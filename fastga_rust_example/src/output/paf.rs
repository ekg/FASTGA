/// PAF format specific utilities

use crate::align::Alignment;

/// PAF record structure
pub struct PafRecord {
    pub query_name: String,
    pub query_length: u64,
    pub query_start: u32,
    pub query_end: u32,
    pub strand: char,
    pub target_name: String,
    pub target_length: u64,
    pub target_start: u32,
    pub target_end: u32,
    pub num_matches: u32,
    pub block_length: u32,
    pub mapping_quality: u8,
    pub tags: Vec<PafTag>,
}

/// PAF optional tags
pub enum PafTag {
    Divergence(f32),      // dv:f
    NumDifferences(u32),  // df:i
    Cigar(String),        // cg:Z
    CS(String),           // cs:Z
}

impl PafRecord {
    /// Format as PAF line
    pub fn to_string(&self) -> String {
        let mut parts = vec![
            self.query_name.clone(),
            self.query_length.to_string(),
            self.query_start.to_string(),
            self.query_end.to_string(),
            self.strand.to_string(),
            self.target_name.clone(),
            self.target_length.to_string(),
            self.target_start.to_string(),
            self.target_end.to_string(),
            self.num_matches.to_string(),
            self.block_length.to_string(),
            self.mapping_quality.to_string(),
        ];

        for tag in &self.tags {
            parts.push(match tag {
                PafTag::Divergence(v) => format!("dv:f:{:.4}", v),
                PafTag::NumDifferences(v) => format!("df:i:{}", v),
                PafTag::Cigar(v) => format!("cg:Z:{}", v),
                PafTag::CS(v) => format!("cs:Z:{}", v),
            });
        }

        parts.join("\t")
    }
}

impl From<&Alignment> for PafRecord {
    fn from(alignment: &Alignment) -> Self {
        PafRecord {
            query_name: String::new(), // Would be filled from GDB
            query_length: 0,
            query_start: alignment.query_start,
            query_end: alignment.query_end,
            strand: if alignment.is_forward() { '+' } else { '-' },
            target_name: String::new(), // Would be filled from GDB
            target_length: 0,
            target_start: alignment.target_start,
            target_end: alignment.target_end,
            num_matches: (alignment.identity * alignment.length() as f32) as u32,
            block_length: alignment.length(),
            mapping_quality: 255,
            tags: vec![
                PafTag::Divergence(1.0 - alignment.identity),
            ],
        }
    }
}