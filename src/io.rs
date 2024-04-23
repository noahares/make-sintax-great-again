use anyhow::{bail, Context, Result};
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::{io::Write, path::PathBuf};

fn normalized_ratio(s: &str) -> Result<f64> {
    let ratio: f64 = s
        .parse()
        .with_context(|| format!("`{s}` isn't a valid fraction"))?;
    if (0.0..=1.0).contains(&ratio) {
        Ok(ratio)
    } else {
        bail!("Fraction is not in range {:.2}-{:.2}", 0.0, 1.0)
    }
}

fn positive_usize(s: &str) -> Result<usize> {
    let value: usize = s
        .parse()
        .with_context(|| format!("`{s}` isn't a valid usize"))?;
    if value > 0 {
        Ok(value)
    } else {
        bail!("Value should be positive")
    }
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the database fasta file
    #[arg(short, long)]
    pub database_path: PathBuf,
    /// Path to the query file
    #[arg(short = 'i', long)]
    pub query_file: PathBuf,
    /// Number of 8-mers
    #[arg(short = 'k', long, default_value_t = 32, value_parser = positive_usize)]
    pub num_k_mers: usize,
    /// Confidence threshold
    #[arg(short = 'c', long, default_value_t = 0.8, value_parser = normalized_ratio)]
    pub min_confidence: f64,
    /// Number of output species per query
    #[arg(short = 'm', long, default_value_t = 5, value_parser = positive_usize)]
    pub max_target_seqs: usize,
    /// If used for mislabling analysis, you want to skip exact sequence matches
    #[arg(long)]
    pub skip_exact_matches: bool,
    /// Number of threads
    /// If 0, uses all available threads
    #[arg(short, long, default_value_t = 0, verbatim_doc_comment)]
    pub threads: usize,
    /// Seed
    #[arg(short, long, default_value_t = 42)]
    pub seed: u64,
    /// Output path
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// confidence output path
    #[arg(short = 'u', long)]
    pub confidence_output: Option<PathBuf>,
    /// log output path
    #[arg(short = 'l', long)]
    pub log_output: Option<PathBuf>,
    /// Force override of existing output files
    #[arg(long)]
    pub redo: bool,
    #[command(flatten)]
    pub verbosity: Verbosity<WarnLevel>,
}

impl Args {
    pub fn get_output(&self) -> Result<Box<dyn Write>> {
        let path = self
            .output
            .clone()
            .unwrap_or(self.query_file.with_extension("sintax.out"));
        if path.is_file() && !self.redo {
            bail!("Output file {} already exists! Please specify another file with -o <PATH> or run with --redo to force overriding existing files!", path.display());
        }
        Ok(std::fs::File::create(path).map(|f| Box::new(f) as Box<dyn Write>)?)
    }
    pub fn get_confidence_output(&self) -> Result<Box<dyn Write>> {
        let path = self.confidence_output.clone().unwrap_or(
            self.query_file
                .with_extension(format!("sintax.conf{}.out", self.min_confidence)),
        );
        if path.is_file() && !self.redo {
            bail!("Output file {} already exists! Please specify another file with -u <PATH> or run with --redo to force overriding existing files!", path.display());
        }
        Ok(std::fs::File::create(path).map(|f| Box::new(f) as Box<dyn Write>)?)
    }
    pub fn get_log_output(&self) -> Result<Box<dyn Write + Send>> {
        let path = self
            .log_output
            .clone()
            .unwrap_or(self.query_file.with_extension("sintax.log"));
        if path.is_file() && !self.redo {
            bail!("Output file {} already exists! Please specify another file with -l <PATH> or run with --redo to force overriding existing files!", path.display());
        }
        Ok(std::fs::File::create(path).map(|f| Box::new(f) as Box<dyn Write + Send>)?)
    }
}
