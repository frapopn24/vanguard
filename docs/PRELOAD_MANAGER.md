# VANGUARD preload manager

The preload manager prepares reusable shader translations and host compute pipeline
states before a normal game run. It also reports the learned graphics pipeline
history stored by normal runs.

## Commands

Build the preloader with the **same build profile and features** that you
will use for the game. Translated-shader cache keys include the diagnostics
feature, so a release preloader will not populate the diagnostics shader cache
used by \`target\\diag\\victory.exe\`.

For normal release gameplay:

\`\`\`powershell
cargo build --release -p victory
$victory = ".\\target\\release\\victory.exe"
\`\`\`

For the diagnostics build used by frame captures:

\`\`\`powershell
cargo build --profile diag -p victory --features diagnostics
$victory = ".\\target\\diag\\victory.exe"
\`\`\`

Then run these commands from the VANGUARD repository root so the intended
\`victory.toml\` is loaded. Do not run the preloader while the game is active;
both processes would compete for GPU compiler time and memory.

Show the current cache without changing it:

\`\`\`powershell
& $victory preload --status
\`\`\`

Scan the configured title and prepare everything currently supported:

\`\`\`powershell
& $victory preload --jobs 8
\`\`\`

Or provide one or more explicit title-data directories:

```powershell
.\target\release\victory.exe preload "C:\Users\Admin\Downloads\YourTitle" --jobs 8
```

Useful options:

- `--jobs N`: number of worker threads. Defaults to the number of logical CPUs.
- `--no-compute-pipelines`: translate and cache shaders but do not create compute PSOs.
- `--report PATH`: write the report somewhere other than the default cache report.
- `--strict`: return a failure exit code if a supported shader translation or requested compute PSO is refused.
- `--status`: report cache sizes and learned graphics pipeline keys without scanning game files.

The previous `victory warm` command remains an alias for `victory preload`.

## What it prepares

### Translated shader cache

The scanner finds console shader containers in supported title files and SGD5
records. Successful vertex, pixel, mesh and compute translations are cached
under the configured cache root's `shaders` directory. The cache fingerprint
includes the translator build and shader settings, so incompatible translations
are not silently reused.

### Compute pipeline states

When compute PSO building is enabled (the default), each discovered supported
compute shader is also used to create a host D3D12 compute pipeline. Successful
PSOs are saved to the same driver pipeline library used by normal runs.

### Graphics pipeline states

A graphics PSO is not identified by vertex and pixel shader bytecode alone. It
also depends on the title's root signature, render-target and depth formats,
input layout, topology, and fixed-function state. The offline shader archive
does not contain a complete, reliable PSO description for every combination,
so the manager does not invent one.

Instead, normal runs learn the exact title pipeline keys. Those keys are stored
in `pipelines/early-pipelines.txt`; the compiled host PSOs live in
`pipelines/host-pipelines.bin`. On a later run, when the title recreates one of
those matching pipeline objects during loading, VANGUARD starts its host pipeline
load/creation early on a worker. The driver library can then satisfy the PSO
without repeating the full compilation.

The D3D12 driver library is tied to the host adapter and driver. If Windows or
the driver rejects an old library, VANGUARD falls back to an empty library and
continues building pipelines as they are requested.

## Cache layout

With the default Windows configuration:

- `%LOCALAPPDATA%\victory\cache\shaders\`: translated shader cache.
- `%LOCALAPPDATA%\victory\cache\pipelines\host-pipelines.bin`: compiled host PSO library.
- `%LOCALAPPDATA%\victory\cache\pipelines\early-pipelines.txt`: pipeline keys learned from earlier runs.
- `%LOCALAPPDATA%\victory\cache\preload-report.txt`: latest preload report.

Use `VICTORY_CACHE_DIR` or `[paths].cache_dir` to choose another cache root.
Use the same cache root for preloading and normal gameplay. If the normal game
launch is made from another working directory and therefore reads a different
`victory.toml`, set `VICTORY_CACHE_DIR` explicitly in both PowerShell sessions. The preload command
does not access or modify game saves.

## Reading the report

The report separates shader discovery, duplicate records, successful and refused
translations by stage, compute PSO outcomes, shader-cache hits/misses, and host
PSOs loaded or created. A refusal is useful diagnostic data, not something the
manager can automatically repair: unsupported instructions, host API rejection,
or unsupported graphics stages require a compatibility-layer fix.

## Performance expectations

Preloading moves work out of the first gameplay use and into an explicit
preparation pass. It can reduce first-use compilation stalls, missing one-shot
passes, and visual hitches where a pipeline was not ready in time.

It does not guarantee higher steady-state FPS. Time spent executing shader
instructions, copying title memory, synchronizing the GPU, or running unsupported
features is not removed by caching. Compare the same scene with and without a
preload run, using the same adapter, driver, cache root and runtime settings.
