# Team task board

GitHub Issues are disabled for this repository, so use this file as the lightweight task board until Issues are enabled.

Each task has a dedicated branch. Keep changes focused and open a pull request into `team-integration`.

## 1. Performance profiling

**Branch:** `work/performance-profiling`

**Priority:** High

**Evidence:** One gameplay log sample reported 2.0 FPS / 503.9 ms per frame: 171.3 ms preparing, 64.3 ms copies (172.4 MB), 13.2 ms texture work (46.1 MB), 233.7 ms GPU time, and 63.3 MB copied back. It also reported 6,426 draws/frame, 337 draws dropped without a host pipeline, and 632 indirect draws.

**Goal:** Establish which costs dominate over several stable samples. Separate CPU preparation, copies, GPU execution, copy-back, pipeline compilation, and draw submission. Implement only measured, isolated optimizations.

**Do not:** Combine changes to shader translation or descriptor heap semantics in this branch.

## 2. Shader translator coverage

**Branch:** `work/shader-translation`

**Priority:** High

**Evidence:** The run log reported rejected shaders for descriptor reads used as data, lane-mask exchange with divergent participation, `v_cvt_f32_f64`, `v_permlanex16_b32`, interpolation handling, and descriptor address/format reads used as data. Host pipeline creation also returned `0x80070057`.

**Goal:** Trace each unsupported operation to translator code and add one focused regression fixture per instruction family. Implement only when semantics can be preserved; retain explicit diagnostics for unsupported cases.

**Do not:** Substitute guessed results merely to make shaders compile.

## 3. Descriptor/view heap capacity

**Branch:** `work/descriptor-heap`

**Priority:** High

**Evidence:** The run log reported a host view heap cap of 1,000,000 while the title requested 1,245,184 views, with views past the cap reading as null.

**Goal:** Determine whether the cap is imposed by D3D12/device limits, the current implementation, or a safety bound. Trace allocation, indexing, lifetimes, and memory use. Consider paging/segmentation only if compatible with the shader indexing model.

**Do not:** Raise the cap blindly. Add bounds and overflow tests.

## 4. Boat/water gameplay simulation

**Branch:** `work/water-and-physics`

**Priority:** Medium

**Evidence:** During the early boat sequence with Jason, the boat sank and the mission failed. This is an observed symptom, not proof that water physics itself is the root cause.

**Goal:** Trace boat/water interaction and determine whether relevant game simulation/HLE updates execute. Separate visual water rendering from gameplay physics; inspect related missing/rejected compute work where evidence supports it.

**Do not:** Add a hard-coded mission workaround before locating the responsible subsystem.

## Shared baseline

- Repository: `frapopn24/victorious`
- Baseline branch: `victory-amd`
- Baseline commit: `f25e7352498e4788583c571c6a37db5b1e4abb8f`
- Integration branch: `team-integration`

Report build success separately from runtime success. Attach concise log excerpts and exact reproduction conditions. Never commit game executables, assets, caches, saves, or generated build output.
