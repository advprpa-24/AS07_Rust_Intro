
pub fn compute_message() -> String {
    "AdvPrPa likes 🦀".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_message() {
        assert_eq!(compute_message(), "AdvPrPa loves 🦀");
    }
}