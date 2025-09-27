/// Traceback operations for alignment reconstruction
pub struct Traceback {
    operations: Vec<TraceOp>,
}

#[derive(Debug, Clone, Copy)]
pub enum TraceOp {
    Match,
    Mismatch,
    Insertion(u32),
    Deletion(u32),
}

impl Traceback {
    pub fn new() -> Self {
        Traceback {
            operations: Vec::new(),
        }
    }

    /// Add a trace operation
    pub fn add_op(&mut self, op: TraceOp) {
        self.operations.push(op);
    }

    /// Convert to CIGAR string
    pub fn to_cigar(&self) -> String {
        let mut cigar = String::new();
        let mut current_op = None;
        let mut count = 0u32;

        for &op in &self.operations {
            let op_char = match op {
                TraceOp::Match | TraceOp::Mismatch => 'M',
                TraceOp::Insertion(_) => 'I',
                TraceOp::Deletion(_) => 'D',
            };

            if current_op == Some(op_char) {
                count += match op {
                    TraceOp::Match | TraceOp::Mismatch => 1,
                    TraceOp::Insertion(n) | TraceOp::Deletion(n) => n,
                };
            } else {
                if let Some(prev_op) = current_op {
                    cigar.push_str(&format!("{}{}", count, prev_op));
                }
                current_op = Some(op_char);
                count = match op {
                    TraceOp::Match | TraceOp::Mismatch => 1,
                    TraceOp::Insertion(n) | TraceOp::Deletion(n) => n,
                };
            }
        }

        if let Some(op) = current_op {
            cigar.push_str(&format!("{}{}", count, op));
        }

        cigar
    }

    /// Convert to extended CIGAR with =/X
    pub fn to_extended_cigar(&self) -> String {
        let mut cigar = String::new();
        let mut current_op = None;
        let mut count = 0u32;

        for &op in &self.operations {
            let op_char = match op {
                TraceOp::Match => '=',
                TraceOp::Mismatch => 'X',
                TraceOp::Insertion(_) => 'I',
                TraceOp::Deletion(_) => 'D',
            };

            if current_op == Some(op_char) {
                count += match op {
                    TraceOp::Match | TraceOp::Mismatch => 1,
                    TraceOp::Insertion(n) | TraceOp::Deletion(n) => n,
                };
            } else {
                if let Some(prev_op) = current_op {
                    cigar.push_str(&format!("{}{}", count, prev_op));
                }
                current_op = Some(op_char);
                count = match op {
                    TraceOp::Match | TraceOp::Mismatch => 1,
                    TraceOp::Insertion(n) | TraceOp::Deletion(n) => n,
                };
            }
        }

        if let Some(op) = current_op {
            cigar.push_str(&format!("{}{}", count, op));
        }

        cigar
    }
}

impl Default for Traceback {
    fn default() -> Self {
        Self::new()
    }
}