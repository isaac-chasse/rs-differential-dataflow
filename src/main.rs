use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MultiSet {
    record: String,
    multiplicity: i32,
}

impl MultiSet {
    fn new(record: String, multiplicity: i32) -> MultiSet {
        MultiSet { record: record, multiplicity: multiplicity }
    }
}

#[derive(Debug, Clone)]
struct Collection(Vec<MultiSet>);

impl Collection {
    fn concat(self, other: Collection) -> Collection {
        let mut out: Vec<MultiSet> = vec![];
        out.extend(self.0);
        out.extend(other.0);
        Collection(out)
    }

    fn negate(self) -> Collection {
        let out = self.0
            .into_iter()
            .map(|MultiSet { record, multiplicity }| MultiSet { record, multiplicity: -multiplicity })
            .collect::<Vec<MultiSet>>();
        Collection(out)
    }

    fn map<F>(&self, f: F) -> Collection 
        where F: Fn(&MultiSet) -> MultiSet
    {
        let out = self.0
            .iter()
            .map(|ms| f(ms))
            .collect();
        Collection(out)
    }

    fn filter<F>(&self, f: F) -> Collection
        where F: Fn(&MultiSet) -> bool
    {
        let out = self.0
            .iter()
            .filter(|ms| f(ms))
            .cloned()
            .collect();
        Collection(out)
    }

    #[allow(dead_code)]
    fn reduce<F>(&self, f: F) -> Collection
        where F: Fn(Vec<(String, i32)>) -> Vec<(String, i32)>
    {
        // There is an opportunity to improve this implementation using
        // `map`, `or_default`, `flat_map` etc that can be more efficient
        // and more idiomatic
        let mut keys: HashMap<String, Vec<(String, i32)>> = HashMap::new();

        for multi_set in &self.0 {
            let entry = keys.entry(multi_set.record.clone()).or_default();
            entry.push((multi_set.record.clone(), multi_set.multiplicity));
        }

        // unused `key` variable here can be improved im sure
        let mut out = vec![];
        for (_key, vals) in keys {
            let results = f(vals);
            for (val, multiplicity) in results {
                out.push(MultiSet::new(val, multiplicity));
            }
        }
        Collection(out)
    }

    fn count(&self) -> Collection {
        fn inner_count(vals: Vec<(String, i32)>) -> Vec<(String, i32)> {
            let out = vals.iter().map(|(_, multiplicity)| *multiplicity).sum();
            vec![("count".to_string(), out)]
        }
        self.reduce(inner_count)
    }

    // fn sum(self) -> () {
    //     ()
    // }

    // fn distinct(self) -> () {
    //     ()
    // }

    // fn min(self) -> () {
    //     ()
    // }

    // fn max(self) -> () {
    //     ()
    // }
}

fn main() {
    let ms0 = MultiSet::new(String::from("apple"), 1);
    let ms1 = MultiSet::new(String::from("orange"), 1);
    let ms2 = MultiSet::new(String::from("apple"), 4);
    let ms3 = MultiSet::new(String::from("pear"), 1);
    println!("{:?}\n{:?}\n{:?}\n{:?}", ms0, ms1, ms2, ms3);

    let collection_a = Collection(vec![ms0, ms1]);
    let collection_b = Collection(vec![ms2, ms3]);
    // println!("{:?}", collection_a);
    // println!("{:?}", collection_b);

    let collection_ab = collection_a.clone().concat(collection_b.clone());
    println!("{:?}", collection_ab);

    // let collection_neg_ab = collection_a.concat(collection_b.negate());
    // println!("{:?}", collection_neg_ab);

    // let collection_upp_ab = collection_ab.clone()
    //     .map(|ms| {
    //         MultiSet::new(ms.record.to_uppercase(), ms.multiplicity)
    //     });
    // println!("{:?}", collection_upp_ab);

    // let collection_ftr_ab = collection_ab.clone().filter(|ms| ms.multiplicity > 1);
    // println!("{:?}", collection_ftr_ab);

    let collection_cnt_ab = collection_ab.clone().count();
    println!("{:?}", collection_cnt_ab);
}
