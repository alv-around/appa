use appa::bench_proof;
use clap::Parser;
use p3_examples::parsers::{DftOptions, FieldOptions, MerkleHashOptions, ProofOptions};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The field to use for our proof.
    #[arg(short, long, ignore_case = true, value_enum)]
    field: FieldOptions,

    /// What we are trying to prove.
    #[arg(short, long, ignore_case = true, value_enum)]
    objective: ProofOptions,

    /// The log base 2 of the desired trace length.
    #[arg(short, long)]
    log_trace_length: u8,

    /// The discrete fourier transform to use in the proof.
    #[arg(short, long, ignore_case = true, value_enum, default_value_t = DftOptions::None)]
    discrete_fourier_transform: DftOptions,

    /// The hash function to use when assembling the Merkle tree.
    #[arg(short, long, ignore_case = true, value_enum)]
    merkle_hash: MerkleHashOptions,
}

fn main() {
    let Args {
        field,
        objective,
        log_trace_length,
        discrete_fourier_transform,
        merkle_hash,
    } = Args::parse();

    bench_proof(
        field,
        objective,
        log_trace_length,
        discrete_fourier_transform,
        merkle_hash,
    );
}
