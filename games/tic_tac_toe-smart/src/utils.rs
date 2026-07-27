pub fn format_number(n: usize) -> String {
    if n >= 1_000_000 {
        let val = n as f64 / 1_000_000.0;
        let formatted = format!("{:.1}", val);
        if formatted.ends_with(".0") {
            format!("{}m", &formatted[..formatted.len() - 2])
        } else {
            format!("{}m", formatted)
        }
    } else if n >= 1_000 {
        let val = n as f64 / 1_000.0;
        let formatted = format!("{:.1}", val);
        if formatted.ends_with(".0") {
            format!("{}k", &formatted[..formatted.len() - 2])
        } else {
            format!("{}k", formatted)
        }
    } else {
        n.to_string()
    }
}
