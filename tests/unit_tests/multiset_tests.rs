#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_multiset_clone() {
        use rs_differential_dataflow::multiset::MultiSet;

        let ms1 = MultiSet::new("foo".to_string(), 2);
        let ms2 = MultiSet::new(9, 6);
        let ms3 = MultiSet::new(("foo", 6), 1);

        assert_eq!(ms1, ms1.clone());
        assert_eq!(ms2, ms2.clone());
        assert_eq!(ms3, ms3.clone());
    }

    #[test]
    fn test_multiset_eq() {
        use rs_differential_dataflow::multiset::MultiSet;

        let ms1 = MultiSet::new("foo".to_string(), 2);
        let ms2 = MultiSet::new(9, 6);
        let ms3 = MultiSet::new(("foo", 6), 1);

        assert_eq!(ms1, MultiSet::new("foo".to_string(), 2));
        assert_eq!(ms2, MultiSet::new(9, 6));
        assert_eq!(ms3, MultiSet::new(("foo", 6), 1));
    }
}