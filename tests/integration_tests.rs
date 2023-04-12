#[cfg(test)]
mod tests {
    #[test]
    fn test_add_one() {
        use rs_differential_dataflow::collection::Collection;
        use rs_differential_dataflow::multiset::MultiSet;

        // not fully extensible yet, only has String implementation
        fn add_one(coll: Collection<String>) -> Collection<String> {
            coll.map(|ms| MultiSet { record: ms.record.clone(), multiplicity: ms.multiplicity.clone() + 1 })
        }

        let ms1 = MultiSet::new("foo".to_string(), 2);
        let ms2 = MultiSet::new("bar".to_string(), 6);
        let coll1 = Collection(vec![
            ms1.clone(),
            ms2.clone(),
        ]);
        let result = add_one(coll1);
        assert_eq!(
            result,
            Collection(vec![
                MultiSet::new("foo".to_string(), 3),
                MultiSet::new("bar".to_string(), 7),
            ])
        );
    }
}