# Candle zip 9 security distribution

Derived exclusively from the crates.io candle-core and candle-nn 0.11.0 archives. Tensor implementation, NN operations and existing tests remain upstream. The NPZ/PTH readers propagate the new zip 9 fallible filename APIs instead of assuming valid names. Package identities, repository/readme/provenance metadata, the zip requirement and the neural-network core dependency are the only differences apart from those three fallible filename call sites and focused regression tests. Original upstream license and notices are retained.

zip is pinned to 9.0.4 to address RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369. Distribution names are dnet-candle-core and dnet-candle-nn, each independently selected at 1.0.0-rc.1. Rust library names remain candle_core and candle_nn; the NN dependency resolves to the same core types. This is not an upstream release or an AppCore Runtime module.

Validation: compare all upstream Rust files; CPU core/NN and AppCore all-feature tests; cargo audit and package checks. No advisory ignore, modified version masquerading as upstream, local registry patch or Git dependency is required by consumers. CUDA/Metal/MKL are inherited optional upstream features, not qualified hardware claims.

Rollback restores the prior complete dependency graph only after assessing its known zip advisories; do not silently mix core identities. Retire these distribution pins in favor of official Candle once its supported registry graph uses a patched zip.
