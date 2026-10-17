# Security distribution: dnet-candle-nn 1.0.0-rc.1

This is a downstream security distribution of Hugging Face candle-nn 0.11.0, not an official Candle release. Tensor/NN implementation is unchanged; three ZIP filename call sites propagate zip 9 errors. The core uses zip =9.0.4; neural-network operations use that same core distribution. No archive-extraction vulnerability exceptions are applied. Upstream license and attribution are retained. GPU backends are not qualified by the CPU validation. Prefer the official upstream package when a compatible zip >=9 release is available. See the repository SECURITY-DISTRIBUTION.md for provenance, validation and rollback.

---

# candle-nn
