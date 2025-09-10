pub trait Chain: Clone {}

#[cfg(feature = "mainnet")]
#[derive(Clone)]
pub struct MainNet {}

#[cfg(feature = "mainnet")]
impl Chain for MainNet {}

#[cfg(feature = "testnet")]
#[derive(Clone)]
pub struct TestNet {}

#[cfg(feature = "testnet")]
impl Chain for TestNet {}

#[cfg(feature = "devnet")]
#[derive(Clone)]
pub struct DevNet {}

#[cfg(feature = "devnet")]
impl Chain for DevNet {}
