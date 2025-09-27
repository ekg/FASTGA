pub mod genome;
pub mod index;
pub mod align;
pub mod output;

pub use genome::{GDB, Sequence, Contig, Scaffold};
pub use index::{KmerIndex, Kmer};
pub use align::{Aligner, Alignment, AlignmentChain};
pub use output::OutputFormat;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::genome::GDB;
    pub use crate::index::KmerIndex;
    pub use crate::align::{Aligner, Alignment};
    pub use crate::output::OutputFormat;
}