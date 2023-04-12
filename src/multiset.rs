#[derive(Debug, Clone)]
pub struct MultiSet {
    pub record: String,
    pub multiplicity: i32,
}

impl MultiSet {
    pub fn new(record: String, multiplicity: i32) -> MultiSet {
        MultiSet { record: record, multiplicity: multiplicity }
    }
}

impl Eq for MultiSet {}

impl PartialEq for MultiSet {
    fn eq(&self, other: &Self) -> bool {
        self.record == other.record && self.multiplicity == other.multiplicity
    }
}