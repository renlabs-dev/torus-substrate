use pallet_permission0::{Error, Pallet};
use pallet_permission0_api::{CuratorPermissions, Permission0CuratorApi};
use polkadot_sdk::{
    frame_support::{assert_err, dispatch::DispatchResult},
    frame_system::RawOrigin,
    polkadot_sdk_frame::prelude::OriginFor,
};
use test_utils::*;

fn ensure_curator(origin: OriginFor<Test>, flags: CuratorPermissions) -> DispatchResult {
    Pallet::<Test>::ensure_curator_permission(origin, flags)
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

        make_curator(key, CuratorPermissions::WHITELIST_MANAGE, None);
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
        make_curator(key, CuratorPermissions::WHITELIST_MANAGE, Some(10));
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
