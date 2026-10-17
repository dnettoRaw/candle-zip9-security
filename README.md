# Candle ZIP 9 security distribution

Downstream security distributions of Hugging Face Candle 0.11.0 for consumers
blocked on the vulnerable ZIP 8 dependency. These are not official Candle
releases and are not AppCore Runtime modules.

- `dnet-candle-core = "=1.0.0-rc.1"`
- `dnet-candle-nn = "=1.0.0-rc.1"`

The tensor and neural-network implementation is retained. Core uses ZIP 9.0.4;
three ZIP filename call sites propagate the new fallible API. NN selects the
same core distribution. See [provenance and scope](SECURITY-DISTRIBUTION.md).
Upstream license and notices are retained.

CPU validation is distinct from accelerator certification. No CUDA, Metal,
MKL, model download or production deployment qualification is asserted.
