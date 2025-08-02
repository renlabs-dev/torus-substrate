use pallet_permission0::{Config, CuratorPermissions, Error, Pallet, PermissionId};
use pallet_permission0_api::{CuratorPermissions as ApiCuratorPermissions, Permission0CuratorApi};
use polkadot_sdk::{
    frame_support::{assert_err, dispatch::DispatchResult},
    frame_system::RawOrigin,
    polkadot_sdk_frame::prelude::OriginFor,
    sp_runtime::BoundedBTreeMap,
};
use test_utils::*;

fn ensure_curator(origin: OriginFor<Test>, flags: CuratorPermissions) -> DispatchResult {
    Pallet::<Test>::ensure_curator_permission(
        origin,
        ApiCuratorPermissions::from_bits_retain(flags.bits()),
    )?;
    Ok(())
}

fn root_permissions(
    flags: CuratorPermissions,
) -> BoundedBTreeMap<
    Option<PermissionId>,
    u32,
    <Test as Config>::MaxCuratorSubpermissionsPerPermission,
> {
    let mut map = BoundedBTreeMap::new();
    map.try_insert(None, flags.bits()).unwrap();
    map
}

#[test]
fn delegate_curator_permission_correctly() {
    new_test_ext().execute_with(|| {
        assert_err!(
            Permission0::delegate_curator_permission(
                RawOrigin::Signed(0).into(),
                1,
                root_permissions(CuratorPermissions::all()),
                None,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::NotPermissionRecipient
        );

        let existing_curator = 1;
        delegate_curator_permission(existing_curator, CuratorPermissions::all(), None);

        assert_err!(
            Permission0::delegate_curator_permission(
                RawOrigin::Root.into(),
                existing_curator,
                root_permissions(CuratorPermissions::all()),
                None,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::DuplicatePermissionInBlock
        );

        step_block(1);

        assert_ok!(Permission0::delegate_curator_permission(
            RawOrigin::Root.into(),
            existing_curator,
            root_permissions(CuratorPermissions::all()),
            None,
            pallet_permission0::PermissionDuration::Indefinite,
            pallet_permission0::RevocationTerms::Irrevocable,
            1
        ));

        let key = 0;

        assert_err!(
            Permission0::delegate_curator_permission(
                RawOrigin::Root.into(),
                key,
                root_permissions(CuratorPermissions::from_bits_retain(
                    pallet_permission0::CuratorPermissions::ROOT.bits()
                )),
                None,
                pallet_permission0::PermissionDuration::Indefinite,
                pallet_permission0::RevocationTerms::Irrevocable,
                1
            ),
            Error::<Test>::InvalidCuratorPermissions
        );
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
