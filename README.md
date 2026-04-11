# Paralegal Experiment

This repo includes experiments for Paralegal.

## Environment

1. You should have the Rust installed, version > 1.85 (Now is 1.94.1).
2. Paralegal works on 1.84, it has defined that in `rust-toolchain` so no worry.
3. All crates in the experiments should not require the Rust version higher than 1.84.

## How to Run

Base:

1. `git clone --recursive https://github.com/syrup4u/paralegal.git`
2. `cd paralegal; cargo install --locked --path crates/paralegal-flow`
3. `cargo paralegal-flow --version`

Move to the directory: `guide/deletion-policy`

By commenting out / back in each test settings, run with `bash run.sh`.
