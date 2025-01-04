polkadot_sdk::frame_benchmarking::define_benchmarks!(
    [frame_benchmarking, BaselineBench::<Runtime>]
    [frame_system, SystemBench::<Runtime>]
    [pallet_balances, Balances]
    [pallet_timestamp, Timestamp]
    [pallet_sudo, Sudo]
    [pallet_torus0, Torus0]
    [pallet_emission0, Emission0]
    [pallet_governance, Governance]
);
