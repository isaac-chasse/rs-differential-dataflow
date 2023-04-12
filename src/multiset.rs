use std::cmp::Ordering;

#[derive(Debug)]
pub struct MultiSet<T>
where
    T: Eq + PartialEq + Ord + PartialOrd,
{
    pub record: T,
    pub multiplicity: i32,
}

impl<T> MultiSet<T>
where
    T: Eq + PartialEq + Ord + PartialOrd,
{
    pub fn new(record: T, multiplicity: i32) -> MultiSet<T> {
        MultiSet {
            record: record,
            multiplicity: multiplicity,
        }
    }
}

impl<T> Clone for MultiSet<T> 
where
    T: Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn clone(&self) -> Self {
        MultiSet { record: self.record.clone(), multiplicity: self.multiplicity.clone() }
    }
}

impl<T> PartialEq for MultiSet<T> 
where
    T: Eq + PartialEq + Ord + PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.record == other.record && self.multiplicity == other.multiplicity
    }
}

impl<T> Eq for MultiSet<T>
where
    T: Eq + PartialEq + Ord + PartialOrd,
{}

impl<T> PartialOrd for MultiSet<T> 
where
    T: Eq + PartialEq + Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.record.partial_cmp(&other.record)
    }
}

impl<T> Ord for MultiSet<T> 
where
    T: Eq + PartialEq + Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.record.cmp(&other.record)
    }
}
