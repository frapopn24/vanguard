# Team workflow

This repository is an experimental Windows compatibility/rendering fork. The tested baseline is `victory-amd` at commit `f25e7352498e4788583c571c6a37db5b1e4abb8f`.

## Branch layout

- `victory-amd`: tested baseline; do not push experimental work directly here.
- `team-integration`: shared integration branch. Merge reviewed pull requests here first.
- `work/performance-profiling`: profiling and performance experiments.
- `work/shader-translation`: shader translator compatibility and missing-pipeline work.
- `work/descriptor-heap`: descriptor/view heap limits and indexing.
- `work/water-and-physics`: water/physics investigation.

## Contributor rules

1. Work only on your assigned `work/*` branch. Do not force-push or rewrite shared history.
2. Keep each pull request focused on one issue. Avoid combining unrelated optimizations.
3. Open pull requests against `team-integration`, not `victory-amd`.
4. Before proposing a change, record the starting commit and the exact files changed.
5. Include the build command, build result, relevant log excerpt, and any runtime result. If you cannot test, say so explicitly.
6. Do not commit game executables, game assets, shader caches, local save data, logs containing personal paths, or generated build output.
7. Do not silently change defaults. Put risky behavior behind a named environment variable or configuration switch, with the safe default preserved.
8. Never claim a fix based only on compilation. Distinguish compile success from runtime verification.

## Build baseline

Use the pinned toolchain in `rust-toolchain.toml` (Rust 1.98.1, MSVC target). The standard release build is:

```powershell
cargo build --release -p victory
```

For diagnostic builds:

```powershell
cargo build --profile diag -p victory --features diagnostics
```

## Current observations from the baseline run

- Textures and faces loaded substantially better after shader-cache warming.
- Runtime performance remained very low (roughly 3–5 FPS in the reported gameplay section).
- The boat sequence failed because the boat sank; water/physics behavior needs investigation, but the cause is not yet established.
- One run logged the host view heap cap at 1,000,000 while the title requested 1,245,184 views; views beyond the cap were reported as null.
- Multiple shaders were rejected by the translator, including descriptor-as-data reads, lane-mask operations, double-to-float conversion, and interpolation handling. These are concrete translator-coverage leads, not yet confirmed causes of each visual defect.

## Review checklist

- Is the change isolated and reversible?
- Does it preserve existing behavior by default?
- Are bounds, resource lifetimes, synchronization, and failure paths handled?
- Does the PR explain what was actually verified?
- Does it include before/after log evidence where applicable?
