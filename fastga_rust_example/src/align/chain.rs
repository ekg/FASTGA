use super::Alignment;

/// Chain of alignments
#[derive(Debug, Clone)]
pub struct AlignmentChain {
    pub alignments: Vec<Alignment>,
    pub total_score: f32,
}

impl AlignmentChain {
    pub fn new() -> Self {
        AlignmentChain {
            alignments: Vec::new(),
            total_score: 0.0,
        }
    }

    pub fn add_alignment(&mut self, alignment: Alignment) {
        self.total_score += alignment.score;
        self.alignments.push(alignment);
    }

    pub fn is_empty(&self) -> bool {
        self.alignments.is_empty()
    }

    pub fn len(&self) -> usize {
        self.alignments.len()
    }

    /// Get total span on target
    pub fn target_span(&self) -> Option<(u32, u32)> {
        if self.alignments.is_empty() {
            return None;
        }

        let start = self.alignments.iter().map(|a| a.target_start).min()?;
        let end = self.alignments.iter().map(|a| a.target_end).max()?;
        Some((start, end))
    }

    /// Get total span on query
    pub fn query_span(&self) -> Option<(u32, u32)> {
        if self.alignments.is_empty() {
            return None;
        }

        let start = self.alignments.iter().map(|a| a.query_start).min()?;
        let end = self.alignments.iter().map(|a| a.query_end).max()?;
        Some((start, end))
    }
}

impl Default for AlignmentChain {
    fn default() -> Self {
        Self::new()
    }
}