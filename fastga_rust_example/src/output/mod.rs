use anyhow::Result;
use std::io::Write;

use crate::align::Alignment;
use crate::genome::GDB;

pub mod paf;
pub mod psl;

/// Output format options
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Paf,
    Psl,
    OneCode,
}

/// Write alignments in PAF format
pub fn write_paf(
    alignments: &[Alignment],
    target: &GDB,
    query: &GDB,
    include_cigar: bool,
) -> Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for alignment in alignments {
        let target_contig = &target.contigs[alignment.target_id as usize];
        let query_contig = &query.contigs[alignment.query_id as usize];

        // Basic PAF fields
        write!(
            handle,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t255",
            query_contig.name,                    // Query name
            query_contig.length,                  // Query length
            alignment.query_start,                 // Query start
            alignment.query_end,                   // Query end
            if alignment.is_forward() { "+" } else { "-" }, // Strand
            target_contig.name,                   // Target name
            target_contig.length,                 // Target length
            alignment.target_start,                // Target start
            alignment.target_end,                  // Target end
            (alignment.identity * alignment.length() as f32) as u32, // Matches
            alignment.length(),                    // Block length
        )?;

        // Optional tags
        write!(handle, "\tdv:f:{:.4}", 1.0 - alignment.identity)?;

        if include_cigar {
            let cigar = alignment.to_extended_cigar();
            write!(handle, "\tcg:Z:{}", cigar)?;
        }

        writeln!(handle)?;
    }

    Ok(())
}

/// Write alignments in PSL format
pub fn write_psl(
    alignments: &[Alignment],
    target: &GDB,
    query: &GDB,
) -> Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for alignment in alignments {
        let target_contig = &target.contigs[alignment.target_id as usize];
        let query_contig = &query.contigs[alignment.query_id as usize];

        // PSL format (simplified)
        writeln!(
            handle,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            (alignment.identity * alignment.length() as f32) as u32, // Matches
            0,                                     // Mismatches
            0,                                     // Rep matches
            0,                                     // N's
            0,                                     // Query gap count
            0,                                     // Query gap bases
            0,                                     // Target gap count
            0,                                     // Target gap bases
            if alignment.is_forward() { "+" } else { "-" }, // Strand
            query_contig.name,                     // Query name
            query_contig.length,                   // Query size
            alignment.query_start,                 // Query start
            alignment.query_end,                   // Query end
            target_contig.name,                   // Target name
            target_contig.length,                 // Target size
            alignment.target_start,                // Target start
            alignment.target_end,                  // Target end
            1,                                     // Block count
            alignment.query_length(),              // Block sizes
            alignment.query_start,                 // Query starts
            alignment.target_start,                // Target starts
        )?;
    }

    Ok(())
}

/// Write alignments in ONE-code format
pub fn write_one_code(
    _alignments: &[Alignment],
    _target: &GDB,
    _query: &GDB,
) -> Result<()> {
    // Placeholder for ONE-code format
    anyhow::bail!("ONE-code format not yet implemented")
}