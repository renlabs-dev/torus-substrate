pub trait Chain {}

#[cfg(feature = "mainnet")]
pub struct MainNet {}

#[cfg(feature = "mainnet")]
impl Chain for MainNet {}

#[cfg(feature = "testnet")]
pub struct TestNet {}

#[cfg(feature = "testnet")]
impl Chain for TestNet {}
