
#[cfg(test)]
mod test {
    use learning_rust;

    #[test]
    fn test_add() {
        assert_eq!(learning_rust::print_labeled_measurement(6, 'h'), "6h");
    }
}