#![allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]

use std::collections::BTreeMap;

use pallet_permission0::{AccumulatedStreamAmounts, PermissionScope};
use pallet_permission0_api::{generate_funnel_stream_id, generate_root_stream_id};
use polkadot_sdk::sp_runtime::{BoundedBTreeMap, Percent};
use test_utils::*;

fn new_test_ext() -> polkadot_sdk::sp_io::TestExternalities {
    new_test_ext_with_block(1)
}

fn setup_permission_with_streams(
    delegator: AccountId,
    recipient: AccountId,
    stream_percentages: Vec<(Percent, u128)>,
    enable_funnel: bool,
) -> (
    pallet_permission0::PermissionId,
    Vec<pallet_permission0::StreamId>,
) {
    zero_min_burn();

    let stream_ids: Vec<_> = (0..stream_percentages.len())
        .map(|i| generate_root_stream_id(&(delegator + i as u32)))
        .collect();

    let mut streams = BTreeMap::new();
    for (stream_id, (pct, _)) in stream_ids.iter().zip(&stream_percentages) {
        streams.insert(*stream_id, *pct);
    }

    let mut recipients = BoundedBTreeMap::new();
    recipients.try_insert(recipient, u16::MAX).unwrap();

    assert_ok!(Permission0::delegate_stream_permission(
        get_origin(delegator),
        recipients,
        pallet_permission0::StreamAllocation::Streams(streams.try_into().unwrap()),
        pallet_permission0::DistributionControl::Manual,
        pallet_permission0::PermissionDuration::Indefinite,
        pallet_permission0::RevocationTerms::RevocableByDelegator,
        pallet_permission0::EnforcementAuthority::None,
        None,
        None,
        enable_funnel,
    ));

    let permission_id = System::events()
        .into_iter()
        .rev()
        .find_map(|record| {
            if let RuntimeEvent::Permission0(pallet_permission0::Event::PermissionDelegated {
                delegator: event_delegator,
                permission_id,
            }) = record.event
            {
                (event_delegator == delegator).then_some(permission_id)
            } else {
                None
            }
        })
        .expect("PermissionDelegated event not found");

    for (stream_id, (_, amount)) in stream_ids.iter().zip(&stream_percentages) {
        if *amount > 0 {
            AccumulatedStreamAmounts::<Test>::set(
                (delegator, stream_id, permission_id),
                Some(*amount),
            );
        }
    }

    (permission_id, stream_ids)
}

fn get_accumulated_total(
    delegator: AccountId,
    permission_id: pallet_permission0::PermissionId,
    stream_ids: &[pallet_permission0::StreamId],
) -> u128 {
    stream_ids
        .iter()
        .filter_map(|sid| AccumulatedStreamAmounts::<Test>::get((delegator, sid, permission_id)))
        .sum()
}

#[test]
fn enable_funnel_with_pre_accumulated_streams() {
    new_test_ext().execute_with(|| {
        let delegator = alice();
        let recipient = bob();

        let amounts = [as_tors(5), as_tors(3), as_tors(2)];
        let stream_percentages = vec![
            (Percent::from_percent(50), amounts[0]),
            (Percent::from_percent(30), amounts[1]),
            (Percent::from_percent(20), amounts[2]),
        ];

        let (permission_id, stream_ids) =
            setup_permission_with_streams(delegator, recipient, stream_percentages, false);

        let total_accumulated: u128 = amounts.iter().sum();
        assert_eq!(
            get_accumulated_total(delegator, permission_id, &stream_ids),
            total_accumulated
        );

        assert_ok!(Permission0::update_stream_permission(
            get_origin(delegator),
            permission_id,
            None,
            None,
            None,
            None,
            None,
            Some(true),
        ));

        let PermissionScope::Stream(scope) =
            pallet_permission0::Permissions::<Test>::get(permission_id)
                .unwrap()
                .scope
        else {
            panic!("Expected stream scope");
        };

        assert!(!scope.funnels.is_empty());
        let derived_stream = scope.funnels[0].derived_stream;
        assert_eq!(derived_stream, generate_funnel_stream_id(&permission_id, 1));

        let recipient_balance_before = get_balance(recipient);

        assert_ok!(Permission0::execute_permission(
            get_origin(delegator),
            permission_id
        ));

        assert_eq!(
            get_balance(recipient),
            recipient_balance_before + total_accumulated
        );

        for stream_id in &stream_ids {
            assert_eq!(
                AccumulatedStreamAmounts::<Test>::get((delegator, stream_id, permission_id)),
                Some(0)
            );
        }
    });
}

#[test]
fn single_level_funnel_chain() {
    new_test_ext().execute_with(|| {
        let delegator_a = alice();
        let recipient_intermediate = bob();

        let amounts_a = [as_tors(10), as_tors(5)];
        let (permission_a, stream_ids_a) = setup_permission_with_streams(
            delegator_a,
            recipient_intermediate,
            vec![
                (Percent::from_percent(60), amounts_a[0]),
                (Percent::from_percent(40), amounts_a[1]),
            ],
            true,
        );

        let PermissionScope::Stream(scope_a) =
            pallet_permission0::Permissions::<Test>::get(permission_a)
                .unwrap()
                .scope
        else {
            panic!("Expected stream scope");
        };

        let derived_stream_a = scope_a.funnels[0].derived_stream;

        let final_recipient = charlie();
        let mut streams_b = BTreeMap::new();
        streams_b.insert(derived_stream_a, Percent::from_percent(100));

        let mut recipients_b = BoundedBTreeMap::new();
        recipients_b.try_insert(final_recipient, u16::MAX).unwrap();

        assert_ok!(Permission0::delegate_stream_permission(
            get_origin(recipient_intermediate),
            recipients_b,
            pallet_permission0::StreamAllocation::Streams(streams_b.try_into().unwrap()),
            pallet_permission0::DistributionControl::Manual,
            pallet_permission0::PermissionDuration::Indefinite,
            pallet_permission0::RevocationTerms::RevocableByDelegator,
            pallet_permission0::EnforcementAuthority::None,
            None,
            None,
            false,
        ));

        let permission_b = System::events()
            .into_iter()
            .rev()
            .find_map(|record| {
                if let RuntimeEvent::Permission0(pallet_permission0::Event::PermissionDelegated {
                    delegator: event_delegator,
                    permission_id,
                }) = record.event
                {
                    (event_delegator == recipient_intermediate).then_some(permission_id)
                } else {
                    None
                }
            })
            .expect("PermissionDelegated event not found");

        let total_a: u128 = amounts_a.iter().sum();

        assert_ok!(Permission0::execute_permission(
            get_origin(delegator_a),
            permission_a
        ));

        assert_eq!(
            AccumulatedStreamAmounts::<Test>::get((
                recipient_intermediate,
                derived_stream_a,
                permission_b
            )),
            Some(total_a)
        );

        let final_recipient_balance_before = get_balance(final_recipient);

        assert_ok!(Permission0::execute_permission(
            get_origin(recipient_intermediate),
            permission_b
        ));

        assert_eq!(
            get_balance(final_recipient),
            final_recipient_balance_before + total_a
        );

        for stream_id in &stream_ids_a {
            assert_eq!(
                AccumulatedStreamAmounts::<Test>::get((delegator_a, stream_id, permission_a)),
                Some(0)
            );
        }

        assert_eq!(
            AccumulatedStreamAmounts::<Test>::get((
                recipient_intermediate,
                derived_stream_a,
                permission_b
            )),
            Some(0)
        );
    });
}

#[test]
fn disable_funnel_mid_accumulation() {
    new_test_ext().execute_with(|| {
        let delegator = alice();
        let recipient = bob();

        let initial_amounts = [as_tors(10), as_tors(5)];
        let (permission_id, stream_ids) = setup_permission_with_streams(
            delegator,
            recipient,
            vec![
                (Percent::from_percent(50), initial_amounts[0]),
                (Percent::from_percent(50), initial_amounts[1]),
            ],
            true,
        );

        let PermissionScope::Stream(scope) =
            pallet_permission0::Permissions::<Test>::get(permission_id)
                .unwrap()
                .scope
        else {
            panic!("Expected stream scope");
        };

        let derived_stream = scope.funnels[0].derived_stream;

        assert_ok!(Permission0::update_stream_permission(
            get_origin(delegator),
            permission_id,
            None,
            None,
            None,
            None,
            None,
            Some(false),
        ));

        let PermissionScope::Stream(scope_after_disable) =
            pallet_permission0::Permissions::<Test>::get(permission_id)
                .unwrap()
                .scope
        else {
            panic!("Expected stream scope");
        };

        assert!(scope_after_disable.funnels.is_empty());

        for (stream_id, amount) in stream_ids.iter().zip(&[as_tors(2), as_tors(3)]) {
            AccumulatedStreamAmounts::<Test>::mutate(
                (delegator, stream_id, permission_id),
                |acc| {
                    *acc = Some(acc.unwrap_or_default() + amount);
                },
            );
        }

        let recipient_balance_before = get_balance(recipient);

        assert_ok!(Permission0::execute_permission(
            get_origin(delegator),
            permission_id
        ));

        let total_distributed: u128 = initial_amounts.iter().sum::<u128>() + as_tors(5);
        assert_eq!(
            get_balance(recipient),
            recipient_balance_before + total_distributed
        );

        for stream_id in &stream_ids {
            assert_eq!(
                AccumulatedStreamAmounts::<Test>::get((delegator, stream_id, permission_id)),
                Some(0)
            );
        }

        assert_ok!(Permission0::update_stream_permission(
            get_origin(delegator),
            permission_id,
            None,
            None,
            None,
            None,
            None,
            Some(true),
        ));

        let PermissionScope::Stream(scope_after_reenable) =
            pallet_permission0::Permissions::<Test>::get(permission_id)
                .unwrap()
                .scope
        else {
            panic!("Expected stream scope");
        };

        assert!(!scope_after_reenable.funnels.is_empty());
        let new_derived_stream = scope_after_reenable.funnels[0].derived_stream;

        assert_eq!(
            new_derived_stream, derived_stream,
            "Re-enabled funnel should create new derived stream with same ID"
        );
        assert_eq!(
            new_derived_stream,
            generate_funnel_stream_id(&permission_id, 1)
        );
    });
}
