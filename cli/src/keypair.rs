use std::str::FromStr;

use bip39::Mnemonic;
use sp_core::{
    bytes::from_hex,
    crypto::{default_ss58_version, Ss58Codec},
    ecdsa, ed25519, sr25519, ByteArray, Pair,
};
use torus_client::subxt::{
    tx::Signer,
    utils::{to_hex, AccountId32, MultiSignature},
    PolkadotConfig,
};

use crate::store::Key;

#[allow(clippy::large_enum_variant)]
pub enum Keypair {
    ED25519(ed25519::Pair),
    SR25519(sr25519::Pair),
    Ecdsa(ecdsa::Pair),
}

impl Keypair {
    pub const fn crypto_type(&self) -> u8 {
        match self {
            Keypair::ED25519(_) => 0,
            Keypair::SR25519(_) => 1,
            Keypair::Ecdsa(_) => 2,
        }
    }

    pub fn account(&self) -> AccountId32 {
        AccountId32::from_str(&self.ss58_address()).unwrap()
    }

    pub fn hex_public_key(&self) -> String {
        match self {
            Keypair::ED25519(keypair) => to_hex(keypair.public().as_slice()),
            Keypair::SR25519(keypair) => to_hex(keypair.public().as_slice()),
            Keypair::Ecdsa(keypair) => to_hex(keypair.public().as_slice()),
        }
    }

    pub fn hex_private_key(&self) -> String {
        match self {
            Keypair::ED25519(keypair) => to_hex(&keypair.to_raw_vec()[..]),
            Keypair::SR25519(keypair) => to_hex(&keypair.to_raw_vec()[..]),
            Keypair::Ecdsa(keypair) => to_hex(&keypair.to_raw_vec()[..]),
        }
    }

    pub fn ss58_address(&self) -> String {
        match self {
            Keypair::ED25519(keypair) => keypair
                .public()
                .to_ss58check_with_version(default_ss58_version()),
            Keypair::SR25519(keypair) => keypair
                .public()
                .to_ss58check_with_version(default_ss58_version()),
            Keypair::Ecdsa(keypair) => keypair
                .public()
                .to_ss58check_with_version(default_ss58_version()),
        }
    }

    pub fn from_key(key: Key, password: Option<&str>) -> anyhow::Result<Self> {
        if key.encrypted {
            anyhow::bail!("the key must be decrypted before becoming a keypair")
        }

        match key.crypto_type {
            0 => {
                if let Some(mnemonic) = key.mnemonic {
                    Ok(Self::ED25519(
                        ed25519::Pair::from_phrase(&mnemonic, password)?.0,
                    ))
                } else if let Some(seed) = key.seed_hex {
                    Ok(Self::ED25519(ed25519::Pair::from_seed(
                        &from_hex(&seed)?[0..32].try_into()?,
                    )))
                } else {
                    unimplemented!()
                }
            }
            1 => {
                if let Some(mnemonic) = key.mnemonic {
                    Ok(Self::SR25519(
                        sr25519::Pair::from_phrase(&mnemonic, password)?.0,
                    ))
                } else if let Some(seed) = key.seed_hex {
                    Ok(Self::SR25519(sr25519::Pair::from_seed(
                        &from_hex(&seed)?[0..32].try_into()?,
                    )))
                } else {
                    unimplemented!()
                }
            }
            2 => {
                if let Some(mnemonic) = key.mnemonic {
                    Ok(Self::Ecdsa(
                        ecdsa::Pair::from_phrase(&mnemonic, password)?.0,
                    ))
                } else if let Some(seed) = key.seed_hex {
                    Ok(Self::Ecdsa(ecdsa::Pair::from_seed(
                        &from_hex(&seed)?[0..32].try_into()?,
                    )))
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Signer<PolkadotConfig> for Keypair {
    fn account_id(&self) -> <PolkadotConfig as torus_client::subxt::Config>::AccountId {
        self.account()
    }

    fn sign(
        &self,
        signer_payload: &[u8],
    ) -> <PolkadotConfig as torus_client::subxt::Config>::Signature {
        match self {
            Keypair::ED25519(pair) => MultiSignature::Ed25519(pair.sign(signer_payload).0),
            Keypair::SR25519(pair) => MultiSignature::Sr25519(pair.sign(signer_payload).0),
            Keypair::Ecdsa(pair) => MultiSignature::Ecdsa(pair.sign(signer_payload).0),
        }
    }
}

pub fn generate_sr25519_keypair(mnemonic: &Mnemonic) -> anyhow::Result<(Keypair, String)> {
    let (keypair, seed) = sr25519::Pair::from_phrase(&mnemonic.to_string(), None)?;

    let seed = to_hex(seed);

    Ok((Keypair::SR25519(keypair), seed))
}
