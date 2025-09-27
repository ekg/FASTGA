/// PSL format specific utilities

use crate::align::Alignment;

/// PSL record structure
pub struct PslRecord {
    pub matches: u32,
    pub mismatches: u32,
    pub rep_matches: u32,
    pub n_count: u32,
    pub q_gap_count: u32,
    pub q_gap_bases: u32,
    pub t_gap_count: u32,
    pub t_gap_bases: u32,
    pub strand: String,
    pub q_name: String,
    pub q_size: u64,
    pub q_start: u32,
    pub q_end: u32,
    pub t_name: String,
    pub t_size: u64,
    pub t_start: u32,
    pub t_end: u32,
    pub block_count: u32,
    pub block_sizes: Vec<u32>,
    pub q_starts: Vec<u32>,
    pub t_starts: Vec<u32>,
}

impl PslRecord {
    /// Format as PSL line
    pub fn to_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.matches,
            self.mismatches,
            self.rep_matches,
            self.n_count,
            self.q_gap_count,
            self.q_gap_bases,
            self.t_gap_count,
            self.t_gap_bases,
            self.strand,
            self.q_name,
            self.q_size,
            self.q_start,
            self.q_end,
            self.t_name,
            self.t_size,
            self.t_start,
            self.t_end,
            self.block_count,
            self.block_sizes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
            self.q_starts.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
            self.t_starts.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
        )
    }
}

impl From<&Alignment> for PslRecord {
    fn from(alignment: &Alignment) -> Self {
        // Simplified conversion - real implementation would handle gaps properly
        PslRecord {
            matches: (alignment.identity * alignment.length() as f32) as u32,
            mismatches: ((1.0 - alignment.identity) * alignment.length() as f32) as u32,
            rep_matches: 0,
            n_count: 0,
            q_gap_count: 0,
            q_gap_bases: 0,
            t_gap_count: 0,
            t_gap_bases: 0,
            strand: if alignment.is_forward() { "+".to_string() } else { "-".to_string() },
            q_name: String::new(), // Would be filled from GDB
            q_size: 0,
            q_start: alignment.query_start,
            q_end: alignment.query_end,
            t_name: String::new(), // Would be filled from GDB
            t_size: 0,
            t_start: alignment.target_start,
            t_end: alignment.target_end,
            block_count: 1,
            block_sizes: vec![alignment.query_length()],
            q_starts: vec![alignment.query_start],
            t_starts: vec![alignment.target_start],
        }
    }
}