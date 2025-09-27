/// Scaffold information
#[derive(Debug, Clone)]
pub struct Scaffold {
    pub id: u32,
    pub name: String,
    pub length: u64,
    pub first_contig: u32,
    pub last_contig: u32,
}

impl Scaffold {
    pub fn new(id: u32, name: String) -> Self {
        Scaffold {
            id,
            name,
            length: 0,
            first_contig: 0,
            last_contig: 0,
        }
    }

    /// Get number of contigs in this scaffold
    pub fn num_contigs(&self) -> u32 {
        self.last_contig - self.first_contig
    }
}