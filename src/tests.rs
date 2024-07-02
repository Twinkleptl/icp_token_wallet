use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let mut contract = TokenContract::new();
        contract.transfer("Alice".to_string(), "Bob".to_string(), 50).unwrap();
        assert_eq!(contract.get_balance(&"Alice".to_string()), 50);
        assert_eq!(contract.get_balance(&"Bob".to_string()), 50);
    }

    #[test]
    fn test_balance_of() {
        let contract = TokenContract::new();
        assert_eq!(contract.get_balance(&"Alice".to_string()), 0);
    }
}