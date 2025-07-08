use pallet_permission0::{Error, Pallet, PermissionScope, Permissions};
use pallet_permission0_api::{CuratorPermissions, Permission0CuratorApi};
use polkadot_sdk::{
    frame_support::{assert_err, dispatch::DispatchResult},
    frame_system::RawOrigin,
    polkadot_sdk_frame::prelude::OriginFor,
    sp_runtime::traits::BadOrigin,
};
use test_utils::*;

fn ensure_curator(origin: OriginFor<Test>, flags: CuratorPermissions) -> DispatchResult {
    Pallet::<Test>::ensure_curator_permission(origin, flags)?;
    Ok(())
}

#[test]
fn delegate_curator_permission_correctly() {
    new_test_ext().execute_with(|| {
        assert_err!(<Permission0 as Permission0CuratorApi<AccountId, RuntimeOrigin, BlockNumber>>::delegate_curator_permission(
            RawOrigin::Signed(0).into(),
            1,
            CuratorPermissions::all(),
            None,
            Default::default(),
            Default::default(),
        ), BadOrigin);

        let existing_curator = 1;
        delegate_curator_permission(existing_curator, CuratorPermissions::all(), None);

        assert_err!(<Permission0 as Permission0CuratorApi<AccountId, RuntimeOrigin, BlockNumber>>::delegate_curator_permission(
            RawOrigin::Root.into(),
            existing_curator,
            CuratorPermissions::all(),
            None,
            Default::default(),
            Default::default(),
        ), Error::<Test>::DuplicatePermission);

        let key = 0;

        assert_err!(<Permission0 as Permission0CuratorApi<AccountId, RuntimeOrigin, BlockNumber>>::delegate_curator_permission(
            RawOrigin::Root.into(),
            key,
            CuratorPermissions::from_bits_retain(pallet_permission0::CuratorPermissions::ROOT.bits()),
            None,
            Default::default(),
            Default::default(),
        ), Error::<Test>::InvalidCuratorPermissions);

        let id = assert_ok!(<Permission0 as Permission0CuratorApi<AccountId, RuntimeOrigin, BlockNumber>>::delegate_curator_permission(
            RawOrigin::Root.into(),
            key,
            // ugly way to check that ROOT is being filtered out
            CuratorPermissions::from_bits_retain(pallet_permission0::CuratorPermissions::all().bits()),
            None,
            Default::default(),
            Default::default(),
        ));

        let contract = Permissions::<Test>::get(id).unwrap();
        let PermissionScope::Curator(scope) = contract.scope else { unreachable!() };
        assert_eq!(scope.flags, pallet_permission0::CuratorPermissions::all() & !pallet_permission0::CuratorPermissions::ROOT);
    });
}

#[test]
fn ensure_curator_handles_curator_permissions() {
    new_test_ext().execute_with(|| {
        assert_ok!(ensure_curator(
            RawOrigin::Root.into(),
            CuratorPermissions::all()
        ));

        let key = 0;
        assert_err!(
            ensure_curator(
                RawOrigin::Signed(key).into(),
                CuratorPermissions::WHITELIST_MANAGE
            ),
            Error::<Test>::PermissionNotFound
        );

        delegate_curator_permission(key, CuratorPermissions::WHITELIST_MANAGE, None);
        assert_ok!(ensure_curator(
            RawOrigin::Signed(key).into(),
            CuratorPermissions::WHITELIST_MANAGE
        ));

        for flags in [
            CuratorPermissions::APPLICATION_REVIEW,
            CuratorPermissions::PENALTY_CONTROL,
        ] {
            assert_err!(
                ensure_curator(RawOrigin::Signed(key).into(), flags),
                Error::<Test>::PermissionNotFound
            );
        }
    });
}

#[test]
fn ensure_curator_handles_cooldown_correctly() {
    new_test_ext().execute_with(|| {
        let key = 0;
        delegate_curator_permission(key, CuratorPermissions::WHITELIST_MANAGE, Some(10));
        let permission_id = Pallet::<Test>::get_curator_permission(&key).unwrap();

        assert_ok!(ensure_curator(
            RawOrigin::Signed(key).into(),
            CuratorPermissions::WHITELIST_MANAGE
        ));

        Pallet::<Test>::execute_permission(RawOrigin::Root.into(), permission_id).unwrap();

        assert_err!(
            ensure_curator(
                RawOrigin::Signed(key).into(),
                CuratorPermissions::WHITELIST_MANAGE
            ),
            Error::<Test>::PermissionInCooldown
        );

        step_block(10);

        assert_ok!(ensure_curator(
            RawOrigin::Signed(key).into(),
            CuratorPermissions::WHITELIST_MANAGE
        ));
    });
}
