pub fn format_torus(amount: u128) -> String {
    format!("{:.5}", amount as f64 / 10f64.powf(18.0))
}
