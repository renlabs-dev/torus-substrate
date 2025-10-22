use std::str::FromStr;

use anyhow::bail;
use sp_core::H256;
use torus_client::subxt::utils::AccountId32;

use crate::{
    action::permission::{
        Allocation, DistributionControl, Duration, EnforcementAuthority, RevocationTerms,
    },
    cli::{DistributionControlParams, RevocationTermsParams},
    store::get_account,
};

pub fn format_torus(amount: u128) -> String {
    format!("{:.5}", amount as f64 / 10f64.powf(18.0))
}

pub fn to_percent_u8(amount: u32) -> anyhow::Result<u8> {
    if amount > 100 {
        anyhow::bail!("Invalid percent: {amount}. Must be between 0-100.");
    }

    Ok(amount.try_into()?)
}

pub fn parse_recipients(string: &str) -> anyhow::Result<Vec<(AccountId32, u16)>> {
    let mut vec = Vec::new();
    let split = string.split(",");

    for ele in split {
        let split = ele.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            bail!("Invalid recipients format, use `id1:weight1,id2:weight2`");
        }

        let recipient = get_account(split[0])?;
        let weight: u16 = split[1].parse()?;
        vec.push((recipient, weight));
    }

    Ok(vec)
}

pub fn parse_allocation(string: &str) -> anyhow::Result<Allocation> {
    if !string.contains(",") && !string.contains(":") {
        return Ok(Allocation::FixedAmount(string.parse()?));
    }

    let mut vec = Vec::new();

    for ele in string.split(",") {
        let split = ele.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            bail!("Invalid allocation format, use `stream1:percentage1,stream2:percentage2`");
        }

        let stream = H256::from_str(split[0])?;
        let weight: u8 = split[1].parse()?;

        vec.push((stream, weight));
    }

    Ok(Allocation::Streams(vec))
}

pub fn parse_distribution(
    params: &DistributionControlParams,
) -> anyhow::Result<DistributionControl> {
    if let Some(value) = params.automatic_distribution {
        return Ok(DistributionControl::Automatic(value));
    }

    if let Some(value) = params.at_block_distribution {
        return Ok(DistributionControl::AtBlock(value));
    }

    if let Some(value) = params.interval_distribution {
        return Ok(DistributionControl::Interval(value));
    }

    Ok(DistributionControl::Manual)
}

pub fn parse_duration(duration: &Option<u64>) -> anyhow::Result<Duration> {
    match duration {
        Some(value) => Ok(Duration::UntilBlock(*value)),
        None => Ok(Duration::Indefinite),
    }
}

pub fn parse_revocation_terms(params: &RevocationTermsParams) -> anyhow::Result<RevocationTerms> {
    if params.revocable_by_delegator.is_some_and(|b| b) {
        return Ok(RevocationTerms::RevocableByDelegator);
    }

    if let Some(value) = params.revocable_after {
        return Ok(RevocationTerms::RevocableAfter(value));
    }

    if let Some(string) = &params.revocable_by_arbiters {
        if !string.contains(",") {
            bail!("Invalid revocation terms arbiter format, use `arbiter1,arbiter2...,votes`");
        }

        let split = string.split(",").collect::<Vec<_>>();
        let mut accounts = Vec::new();
        let mut required_votes = 0;
        for (idx, ele) in split.iter().enumerate() {
            if idx + 1 == split.len() {
                required_votes = ele.parse()?;
            } else {
                accounts.push(get_account(ele)?);
            }
        }

        return Ok(RevocationTerms::RevocableByArbiters {
            accounts,
            required_votes,
        });
    }

    Ok(RevocationTerms::Irrevocable)
}

pub fn parse_enforcement_authority(
    params: &Option<String>,
) -> anyhow::Result<EnforcementAuthority> {
    if let Some(string) = &params {
        if !string.contains(",") {
            bail!("Invalid enforcement authority format, use `controller1,controller2...,votes`");
        }

        let split = string.split(",").collect::<Vec<_>>();
        let mut accounts = Vec::new();
        let mut required_votes = 0;
        for (idx, ele) in split.iter().enumerate() {
            if idx + 1 == split.len() {
                required_votes = ele.parse()?;
            } else {
                accounts.push(get_account(ele)?);
            }
        }

        return Ok(EnforcementAuthority::ControlledBy {
            controllers: accounts,
            required_votes,
        });
    }

    Ok(EnforcementAuthority::None)
}

pub fn parse_paths(paths: &Vec<String>) -> anyhow::Result<Vec<(Option<H256>, Vec<String>)>> {
    let mut vec = Vec::new();
    for paths in paths {
        let mut paths = paths.clone();

        let permission_id = if paths.contains(":") {
            let split = paths
                .split(":")
                .map(|path| path.to_string())
                .collect::<Vec<_>>();
            if split.len() > 2 || split.is_empty() {
                bail!("Invalid paths format, use `(permissionid:)path1,path2...`");
            }
            paths = split[1].clone();
            Some(H256::from_str(&split[0])?)
        } else {
            None
        };

        let paths = paths
            .split(",")
            .map(|path| path.to_string())
            .collect::<Vec<_>>();
        vec.push((permission_id, paths));
    }

    Ok(vec)
}

pub fn parse_flags(paths: &Option<Vec<String>>) -> anyhow::Result<Vec<(Option<H256>, u32)>> {
    match paths {
        None => Ok(vec![]),
        Some(strings) => {
            let mut vec = Vec::new();
            for ele in strings {
                let mut ele = ele.clone();

                let id = if ele.contains(":") {
                    let split = ele
                        .split(":")
                        .map(|path| path.to_string())
                        .collect::<Vec<_>>();
                    if split.len() > 2 || split.is_empty() {
                        bail!("Invalid paths format, use `(permissionid:)path1,path2...`");
                    }
                    ele = split[1].clone();
                    Some(H256::from_str(&split[0])?)
                } else {
                    None
                };

                let val = ele.parse()?;
                vec.push((id, val));
            }

            Ok(vec)
        }
    }
}
