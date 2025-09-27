/// Alignment between two sequences
#[derive(Debug, Clone)]
pub struct Alignment {
    pub target_id: u32,
    pub target_start: u32,
    pub target_end: u32,
    pub query_id: u32,
    pub query_start: u32,
    pub query_end: u32,
    pub score: f32,
    pub identity: f32,
    pub path: AlignmentPath,
}

/// Alignment path representation
#[derive(Debug, Clone)]
pub enum AlignmentPath {
    Simple,
    Trace(Vec<TracePoint>),
    Cigar(String),
}

#[derive(Debug, Clone)]
pub struct TracePoint {
    pub target_pos: u32,
    pub query_pos: u32,
    pub differences: u16,
}

impl Alignment {
    /// Get alignment length on target
    pub fn target_length(&self) -> u32 {
        self.target_end - self.target_start
    }

    /// Get alignment length on query
    pub fn query_length(&self) -> u32 {
        self.query_end - self.query_start
    }

    /// Get total alignment length
    pub fn length(&self) -> u32 {
        self.target_length().max(self.query_length())
    }

    /// Check if this is a forward strand alignment
    pub fn is_forward(&self) -> bool {
        true // Simplified - real implementation would track strand
    }

    /// Generate CIGAR string
    pub fn to_cigar(&self) -> String {
        match &self.path {
            AlignmentPath::Cigar(cigar) => cigar.clone(),
            _ => {
                // Generate simple CIGAR
                format!("{}M", self.target_length())
            }
        }
    }

    /// Generate extended CIGAR with =/X operators
    pub fn to_extended_cigar(&self) -> String {
        match &self.path {
            AlignmentPath::Cigar(cigar) => {
                // Convert M to =/X if needed
                cigar.clone()
            }
            _ => {
                // Generate simple extended CIGAR
                format!("{}=", self.target_length())
            }
        }
    }
}