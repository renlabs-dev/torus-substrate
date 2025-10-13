pub fn format_torus(amount: u128) -> String {
    format!("{:.5}", amount as f64 / 10f64.powf(18.0))
}

pub fn to_percent_u8(amount: u32) -> anyhow::Result<u8> {
    if amount > 100 {
        anyhow::bail!("Invalid percent: {amount}. Must be between 0-100.");
    }

    Ok(amount.try_into()?)
}
