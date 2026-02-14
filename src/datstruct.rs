#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fasta {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct PathStruct {
    pub pathexpression: String,
    pub pathfasta: String,
    pub windowsize: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FastaStruct {
    id: String,
    seq: String,
}

#[derive(Debug)]
pub struct FastaRecord {
    pub id: String,
    pub sequence: String,
}
