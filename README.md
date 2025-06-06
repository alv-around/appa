# appa

Verifying STARKs with labrador

## RUN

```
RUSTFLAGS="-Ctarget-cpu=native" RUST_BACKTRACE=1 cargo run --bin prove_prime_field_31 --release --features parallel -- --field koala-bear --objective poseidon-2-permutations --log-trace-length 17 --discrete-fourier-transform radix-2-dit-parallel --merkle-hash keccak-f
```
