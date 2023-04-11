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


#[allow(dead_code)]
impl Collection {
    /// Combines two collections into one. `concat` is the same as adding two collections 
    /// together. `concat` can let us copy both elements into one list that outputs a 
    /// (record, multiplicity) pair.
    fn concat(self, other: Collection) -> Collection {
        let mut out: Vec<MultiSet> = vec![];
        out.extend(self.0);
        out.extend(other.0);
        Collection(out)
    }

    /// Multiplies all the multiplicities by -1. You can use `concat` and `negate` together
    /// to substract collections.
    fn negate(self) -> Collection {
        let out = self.0
            .into_iter()
            .map(|MultiSet { record, multiplicity }| MultiSet { record, multiplicity: -multiplicity })
            .collect::<Vec<MultiSet>>();
        Collection(out)
    }

    /// Applies a function `f` to all the records in the collection and produces a new collection
    /// containing `f(record)`.
    fn map<F>(&self, f: F) -> Collection 
        where F: Fn(&MultiSet) -> MultiSet
    {
        let out = self.0
            .iter()
            .map(|ms| f(ms))
            .collect();
        Collection(out)
    }

    /// Applies a function `f` to all the records in the collection and produces a new collection
    /// containing `record if f(record) == true`.
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

    /// This operation is predicated onn a key-value structure. Takes every input in the collection
    /// and applies a function `f` to the multiset of the values associated with that key, returning
    /// a collection containing `(key, f(values associated with key))`. We can also define functions 
    /// built on top of `reduce`, seen below.
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

    /// Returns the number of values associated with each key
    fn count(&self) -> Collection {
        let reduced = self.reduce(|vals| {
            let count = vals.len() as i32;
            vec![(vals[0].0.clone(), count)]
        });
        reduced
    }

    /// Returns the sum of the values associated with each key
    fn sum(&self) -> Collection {
        let reduced = self.reduce(|vals| {
            let sum = vals
                .iter()
                .map(|(_, multiplicity)| multiplicity).sum();
            vec![(vals[0].0.clone(), sum)]
        });
        reduced
    }

    /// returns the distinct set of values associated with each key
    fn distinct(&self) -> Collection {
        let reduced = self.reduce(|vals| {
            let mut distinct: Vec<_> = vals
                .iter()
                .map(|(val, _)| val.clone())
                .collect();
            distinct.sort_unstable();
            distinct.dedup();
            let out = distinct
                .into_iter()
                .map(|val| (val, 1))
                .collect();
            out
        });
        reduced
    }

    /// Produces a normalized, logically equivalent version of the input collection
    /// containing exactly one instance of each record, and no records with a multiplicity
    /// of 0. 
    fn consolidate(&self) -> Collection {
        // tbh I think this is wrong -- currently outputs MultiSet(record, 1) for Collection
        let reduced = self.reduce(|vals| {
            let mut count = 0;
            let mut out = vec![];
            for (record, multiplicity) in vals {
                count += multiplicity;
                if multiplicity > 0 && count == multiplicity {
                    out.push((record.clone(), multiplicity));
                }
            }
            out
        });
        reduced
    }

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

    // let collection_cnt_ab = collection_ab.clone().count();
    // println!("{:?}", collection_cnt_ab);

    // let collection_sum_ab = collection_ab.clone().sum();
    // println!("{:?}", collection_sum_ab);

    // let collection_dst_ab = collection_ab.clone().distinct();
    // println!("{:?}", collection_dst_ab);

    let collection_consolidated = collection_ab.clone().consolidate();
    println!("{:?}", collection_consolidated);
    
}
