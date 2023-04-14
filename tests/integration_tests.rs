#[cfg(test)]
mod tests {
    #[test]
    fn test_add_one() {
        use rs_differential_dataflow::collection::Collection;
        use rs_differential_dataflow::multiset::MultiSet;

        // not fully extensible yet, only has String implementation
        fn add_one_strings(coll: Collection<String>) -> Collection<String> {
            coll.map(|ms| MultiSet { record: ms.record.clone(), multiplicity: ms.multiplicity.clone() + 1 })
        }

        let ms1 = MultiSet::new("foo".to_string(), 2);
        let ms2 = MultiSet::new("bar".to_string(), 6);
        let coll1 = Collection(vec![
            ms1.clone(),
            ms2.clone(),
        ]);
        let result = add_one_strings(coll1);
        assert_eq!(
            result,
            Collection(vec![
                MultiSet::new("foo".to_string(), 3),
                MultiSet::new("bar".to_string(), 7),
            ])
        );
    }

    #[test]
    fn test_fixpoint_computation() {
        use rs_differential_dataflow::collection::Collection;
        use rs_differential_dataflow::multiset::MultiSet;

        // Option 1 is to rebuild this fn to work with Collection<(i32, i32)>
        fn collection_fixpoint(coll: Collection<i32>) -> Collection<i32> {
            coll.map(|ms| MultiSet { record: ms.record.clone() + 1 , multiplicity: ms.multiplicity.clone() })
                .concat(coll)
                .filter(|ms| ms.record <= 5)
                .distinct()
        }

        let collection = Collection::new(
            vec![MultiSet::new(1, 1)]
        );

        let expected = Collection(vec![
            MultiSet { record: 1, multiplicity: 1 }, 
            MultiSet { record: 2, multiplicity: 1 }, 
            MultiSet { record: 3, multiplicity: 1 }, 
            MultiSet { record: 4, multiplicity: 1 }, 
            MultiSet { record: 5, multiplicity: 1 }
        ]);

        let result = collection.iterate(|cl| collection_fixpoint(cl.clone()));
        assert_eq!(result, expected);
    }
}