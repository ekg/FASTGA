use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use std::path::PathBuf;

mod genome;
mod index;
mod align;
mod output;

use crate::genome::GDB;
use crate::index::KmerIndex;
use crate::align::Aligner;
use crate::output::OutputFormat;

#[derive(Parser, Debug)]
#[command(name = "fastga")]
#[command(version = "0.1.0")]
#[command(about = "Fast genome aligner using adaptive k-mer matching")]
struct Args {
    /// Target genome (reference)
    target: PathBuf,

    /// Query genome
    query: PathBuf,

    /// Number of threads
    #[arg(short = 'T', long, default_value = "8")]
    threads: usize,

    /// K-mer size
    #[arg(short, long, default_value = "14")]
    kmer_size: usize,

    /// Frequency cutoff
    #[arg(short, long, default_value = "10")]
    frequency: u32,

    /// Output format
    #[arg(short, long, value_enum, default_value = "paf")]
    output: OutputFormat,

    /// Include CIGAR strings (PAF only)
    #[arg(long)]
    cigar: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Keep index files
    #[arg(short = 'k', long)]
    keep_index: bool,

    /// Temporary directory
    #[arg(short = 'P', long)]
    temp_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    if args.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Warn)
            .init();
    }

    // Set up thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()?;

    info!("Loading target genome: {:?}", args.target);
    let target = load_or_build_genome(&args.target)?;

    info!("Loading query genome: {:?}", args.query);
    let query = load_or_build_genome(&args.query)?;

    // Build or load indices
    let pb = create_progress_bar("Building indices");

    info!("Building k-mer index for target");
    let target_index = KmerIndex::build(&target, args.kmer_size, args.frequency)?;
    pb.inc(50);

    info!("Building k-mer index for query");
    let query_index = KmerIndex::build(&query, args.kmer_size, args.frequency)?;
    pb.finish_with_message("Indices built");

    // Perform alignment
    let pb = create_progress_bar("Finding alignments");
    let aligner = Aligner::new()
        .with_threads(args.threads)
        .with_min_chain_score(85)
        .with_min_alignment_length(100);

    let alignments = aligner.align_genomes(&target, &target_index, &query, &query_index)?;
    pb.finish_with_message(format!("Found {} alignments", alignments.len()));

    // Output results
    info!("Writing output in {:?} format", args.output);
    match args.output {
        OutputFormat::Paf => {
            output::write_paf(&alignments, &target, &query, args.cigar)?;
        }
        OutputFormat::Psl => {
            output::write_psl(&alignments, &target, &query)?;
        }
        OutputFormat::OneCode => {
            output::write_one_code(&alignments, &target, &query)?;
        }
    }

    // Clean up indices if not keeping
    if !args.keep_index {
        info!("Cleaning up temporary index files");
        // Cleanup code here
    }

    Ok(())
}

fn load_or_build_genome(path: &PathBuf) -> Result<GDB> {
    // Check if .gdb exists alongside the input
    let gdb_path = path.with_extension("gdb");

    if gdb_path.exists() {
        info!("Loading existing GDB: {:?}", gdb_path);
        GDB::load(&gdb_path)
    } else {
        info!("Building GDB from source: {:?}", path);
        let gdb = GDB::from_source(path)?;
        gdb.save(&gdb_path)?;
        Ok(gdb)
    }
}

fn create_progress_bar(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {percent}%")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(msg.to_string());
    pb
}