# Team workstreams

This page coordinates independent investigations for the community fork of [JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd). It is a working list, not a claim that any item is already fixed.

## Ground rules

- Branch from `team-integration`; open pull requests back into `team-integration`.
- Keep changes isolated and reversible.
- Record the commit SHA, hardware, driver, settings, reproduction steps, and evidence.
- Do not commit game files, extracted assets, save data, shader caches, or private logs.
- Treat a correlation as a lead, not proof of root cause.

## Workstreams

### 1. Shader translation and pipeline creation

**Goal:** understand and reduce shader translation failures and host pipeline refusals.

Useful evidence:
- Shader hash and shader stage.
- Full translation error and nearby log context.
- Whether the host pipeline was created, refused, or skipped.
- A minimal test or capture that reproduces the behavior.

Keep one shader instruction or failure class per change where possible.

### 2. Descriptor heap and resource lifetime

**Goal:** investigate descriptor capacity, invalid/null descriptor reads, and resource-view lifetime.

Useful evidence:
- Heap type and requested/available descriptor counts.
- Allocation and lookup path.
- Whether the issue reproduces with diagnostics enabled.
- Validation output and a minimal reproduction.

Avoid changing global caps without measuring the memory and correctness impact.

### 3. Performance profiling

**Goal:** identify the largest repeatable CPU, GPU, synchronization, copy, and memory costs.

Record:
- FPS and frame-time range over a consistent scene and duration.
- CPU and GPU timing breakdowns where available.
- Memory use and copy volume.
- Exact build and environment settings.

Change one variable at a time and retain a known-good baseline.

### 4. Water, vehicles, collision, and gameplay behavior

**Goal:** determine why the boat sequence currently fails and whether the cause is graphics, compute execution, game-side simulation, collision/query behavior, or another compatibility gap.

First steps:
- Reproduce from a known save/checkpoint and record the exact sequence.
- Capture logs immediately before and after the failure.
- Separate visual water rendering from vehicle movement and mission-state behavior.
- Compare relevant compute/pipeline failures without assuming any particular shader hash is responsible.

**Current confidence:** root cause unknown. No specific shader or log warning has been confirmed as the cause.

## Reporting a completed investigation

Use this format in a pull request:

- **Question / hypothesis:**
- **Change made:**
- **Build commit:**
- **Test system:**
- **Reproduction steps:**
- **Before / after evidence:**
- **Limitations / remaining questions:**
