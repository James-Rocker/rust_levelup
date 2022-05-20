

// #[cfg(test)]
// mod test {
    use myswankynewlib;

    #[test]
    fn test_add() {
        assert_eq!(myswankynewlib::print_labeled_measurement(6, 'h'), "6h");
    }
// }