/// Contig information
#[derive(Debug, Clone)]
pub struct Contig {
    pub id: u32,
    pub scaffold_id: u32,
    pub length: u64,
    pub sequence_offset: u64,
    pub name: String,
}

impl Contig {
    pub fn new(id: u32, scaffold_id: u32, length: u64, sequence_offset: u64, name: String) -> Self {
        Contig {
            id,
            scaffold_id,
            length,
            sequence_offset,
            name,
        }
    }
}