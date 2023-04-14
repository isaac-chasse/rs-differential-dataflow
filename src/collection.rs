use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::multiset::MultiSet;

/// A collection of `MultiSet`s, where each `MultiSet` represents a record and its multiplicity.
#[derive(Debug, Clone)]
pub struct  Collection<T: Ord>(pub Vec<MultiSet<T>>);

impl<T: Clone> PartialEq for Collection<T> 
where 
    T: Ord,
{
    /// Returns `true` if `self` and `other` have the same elements in the same order.
    fn eq(&self, other: &Self) -> bool {
        let mut self_vec = self.0.clone();
        let mut other_vec = other.0.clone();
        self_vec.sort_by_key(|ms| ms.record.clone());
        other_vec.sort_by_key(|ms| ms.record.clone());
        self_vec == other_vec
    }
}

impl<T: Clone> Eq for Collection<T> 
where 
    T: Ord,
{}

#[allow(dead_code)]
impl<T: Clone> Collection<T> 
where 
    T: Ord,
{
    /// Instantiates a new Collection<T> from a Vec of MultiSet<T>. All <T> must be equivalent currently.
    pub fn new(multiset_vec: Vec<MultiSet<T>>) -> Collection<T> {
        Collection(multiset_vec)
    }

    /// Combines two collections into one. `concat` is the same as adding two collections
    /// together. `concat` can let us copy both elements into one list that outputs a
    /// (record, multiplicity) pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll1 = Collection(vec![MultiSet::new("a".to_string(), 1), MultiSet::new("b".to_string(), 2)]);
    /// let coll2 = Collection(vec![MultiSet::new("c".to_string(), 3), MultiSet::new("d".to_string(), 4)]);
    /// let result = coll1.concat(coll2);
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    ///     MultiSet::new("c".to_string(), 3),
    ///     MultiSet::new("d".to_string(), 4),
    /// ]));
    /// ```
    pub fn concat(self, other: Collection<T>) -> Collection<T> {
        let mut out: Vec<MultiSet<T>> = vec![];
        out.extend(self.0);
        out.extend(other.0);
        Collection(out)
    }

    /// Multiplies all the multiplicities by -1. You can use `concat` and `negate` together
    /// to substract collections.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![MultiSet::new("a".to_string(), 1), MultiSet::new("b".to_string(), -2)]);
    /// let result = coll.negate();
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), -1),
    ///     MultiSet::new("b".to_string(), 2),
    /// ]));
    /// ```
    pub fn negate(self) -> Collection<T> {
        let out = self.0
            .into_iter()
            .map(|MultiSet { record, multiplicity }| MultiSet { record, multiplicity: -multiplicity })
            .collect::<Vec<MultiSet<T>>>();
        Collection(out)
    }

    /// Applies a function `f` to all the records in the collection and produces a new collection
    /// containing `f(record)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![MultiSet::new("a".to_string(), 1), MultiSet::new("b".to_string(), 2)]);
    /// let result = coll.map(|MultiSet { record, multiplicity }| MultiSet { record: record.to_uppercase(), multiplicity: *multiplicity });
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("A".to_string(), 1),
    ///     MultiSet::new("B".to_string(), 2),
    /// ]));
    /// ```
    pub fn map<F>(&self, f: F) -> Collection<T> 
        where F: Fn(&MultiSet<T>) -> MultiSet<T>
    {
        let out = self.0
            .iter()
            .map(|ms| f(ms))
            .collect();
        Collection(out)
    }

    /// Applies a function `f` to all the records in the collection and produces a new collection
    /// containing `record if f(record) == true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![MultiSet::new("a".to_string(), 1), MultiSet::new("b".to_string(), 2)]);
    /// let result = coll.filter(|MultiSet { record, multiplicity }| record == "b");
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("b".to_string(), 2),
    /// ]));
    /// ```
    pub fn filter<F>(&self, f: F) -> Collection<T>
        where F: Fn(&MultiSet<T>) -> bool
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
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 4),
    /// ]);
    /// let result = coll.reduce(|vals| {
    ///     let sum = vals.iter().map(|(_, multiplicity)| multiplicity).sum();
    ///     vec![(vals[0].0.clone(), sum)]
    /// });
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 4),
    ///     MultiSet::new("b".to_string(), 6),
    /// ]));
    /// ```
    pub fn reduce<F>(&self, f: F) -> Collection<T>
    where
        F: Fn(Vec<(T, i32)>) -> Vec<(T, i32)>,
        T: Eq + std::hash::Hash,
    {
        let mut keys: HashMap<T, Vec<(T, i32)>> = HashMap::new();

        for multi_set in &self.0 {
            let entry = keys.entry(multi_set.record.clone()).or_default();
            entry.push((multi_set.record.clone(), multi_set.multiplicity));
        }

        let mut out = vec![];
        for (_key, vals) in keys {
            let results = f(vals);
            for (val, multiplicity) in results {
                out.push(MultiSet::new(val, multiplicity));
            }
        }

        // Sort the resulting Collection by record and multiplicity
        out.sort_unstable_by(|a, b| {
            a.record
                .cmp(&b.record)
                .then(a.multiplicity.cmp(&b.multiplicity))
        });

        Collection(out)
    }

    /// Returns the number of values associated with each key.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 4),
    /// ]);
    /// let result = coll.count();
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 2),
    ///     MultiSet::new("b".to_string(), 2),
    /// ]));
    /// ```
    pub fn count(&self) -> Collection<T>
    where 
        T: Hash
    {
        let reduced = self.reduce(|vals| {
            let count = vals.len() as i32;
            vec![(vals[0].0.clone(), count)]
        });
        reduced
    }

    /// Returns the sum of the values associated with each key.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 4),
    /// ]);
    /// let result = coll.sum();
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 4),
    ///     MultiSet::new("b".to_string(), 6),
    /// ]));
    /// ```
    pub fn sum(&self) -> Collection<T> 
    where 
        T: Hash
    {
        let reduced = self.reduce(|vals| {
            let sum = vals
                .iter()
                .map(|(_, multiplicity)| multiplicity).sum();
            vec![(vals[0].0.clone(), sum)]
        });
        reduced
    }

    /// Returns a collection containing the distinct set of values associated with each key.
    ///
    /// This function groups elements in the collection by their keys, and then returns a new collection
    /// where each distinct value associated with each key is represented exactly once. The resulting
    /// collection is unordered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 4),
    /// ]);
    /// let result = coll.distinct();
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 1),
    /// ]));
    /// ```
    ///
    /// # Notes
    ///
    /// - The order of the elements in the resulting collection is not guaranteed.
    /// - If the input collection is empty, the resulting collection will also be empty.
    ///
    pub fn distinct(&self) -> Collection<T> 
    where 
        T: Hash
    {
        let reduced = self.reduce(|vals| {
            let mut distinct = std::collections::HashSet::new();
            for (val, _) in vals {
                distinct.insert(val.clone());
            }
            let out = distinct.into_iter().map(|val| (val, 1)).collect();
            out
        });
        reduced
    }

    /// Produces a normalized, logically equivalent version of the input collection
    /// containing exactly one instance of each record, and no records with a multiplicity
    /// of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 0),
    ///     MultiSet::new("a".to_string(), -1),
    ///     MultiSet::new("a".to_string(), 2),
    /// ]);
    /// ```
    pub fn consolidate(&self) -> Collection<T> 
    where 
        T: Hash
    {
        // BUG: tbh I think this is wrong -- currently outputs MultiSet(record, 1) for Collection
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

    /// Takes two input collections, and for all `(x, y)` in the first collection, and all
    /// `(x, z)` in the second collection, produces `(x, (y, z))` as output.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll1 = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    /// ]);
    /// let coll2 = Collection(vec![
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 4),
    /// ]);
    /// let result = coll1.join(&coll2);
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 8),
    /// ]));
    /// ```
    pub fn join(&self, other: &Collection<T>) -> Collection<T> 
    where 
        T: Hash
    {
        let out = self.0.iter()
            .flat_map(|ms1| other.0.iter().filter(move |ms2| ms1.record == ms2.record)
                .map(move |ms2| MultiSet::new(ms1.record.clone(), ms1.multiplicity * ms2.multiplicity)))
            .collect::<Vec<_>>();

        let mut records = HashSet::new();
        let deduped_out = out.into_iter()
            .filter(|ms| records.insert(ms.record.clone()))
            .collect::<Vec<_>>();
        
        Collection(deduped_out)
    }

    /// This function takes one input collection and repeatedly applies a function `f` to the
    /// input until the output stops changing. `f` can be any combination of the functional
    /// operations defined withing `impl Collection`, including other nested calls to `iterate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_differential_dataflow::collection::Collection;
    /// use rs_differential_dataflow::multiset::MultiSet;
    ///
    /// let coll = Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 2),
    ///     MultiSet::new("a".to_string(), 3),
    ///     MultiSet::new("b".to_string(), 4),
    /// ]);
    /// let result = coll.iterate(|coll| coll.distinct().sum().count());
    /// assert_eq!(result, Collection(vec![
    ///     MultiSet::new("a".to_string(), 1),
    ///     MultiSet::new("b".to_string(), 1),
    /// ]));
    /// ```
    pub fn iterate<F>(&self, f: F) -> Collection<T>
    where 
        F: Fn(&Collection<T>) -> Collection<T>,
    {
        let mut curr = Collection(self.0.clone());
        loop {
            let result = f(&curr);
            if result.0 == curr.0 {
                break;
            }
            curr = result;
        }
        curr
    }

    // fn min(self) -> () {
    //     ()
    // }

    // fn max(self) -> () {
    //     ()
    // }
}