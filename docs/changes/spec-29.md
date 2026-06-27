# Spec 29 Changelog

This release migrates the allocator role to the current sudo key.

## 1. Extrinsics

No extrinsics were added, changed, or removed.

## 2. Events

No events were added, changed, or removed.

## 3. Storage Items

No storage items were added, changed, or removed.

## 4. Runtime Migration

- Runtime `spec_version` is bumped from `28` to `29`.
- `governance` storage version is bumped from `5` to `6`.
- The migration reads the current sudo key and makes it the only governance allocator.
- The governance allocator set is replaced with the sudo key.
- The sudo allocator is whitelisted, funded with the current registration burn from the DAO treasury, and registered through the normal torus0 registration path with the fixed name `new_allocator`.
- Stake to prior allocators is moved to the sudo allocator before prior allocator agents are deregistered; prior allocator outbound stake is unstaked back to free balance first.
- Prior allocator agents are deregistered through the torus0 deregistration path after their inbound stake has moved.
