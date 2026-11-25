# Windows startup and diagnostics

Run these commands in PowerShell from the repository root (the folder containing the root `Cargo.toml`). The diagnostics build is the build used for frame-operation traces.

## 1. Configure the game directory

Create the local config file once:

```powershell
if (-not (Test-Path .\victory.toml)) {
    Copy-Item .\victory.example.toml .\victory.toml
}
notepad .\victory.toml
```

Edit the `[paths]` section. Set `game_dir` to the directory containing your game files, for example:

```toml
[paths]
game_dir = 'D:\Games\YourTitle'
```

This is a TOML literal string (single quotes), so use ordinary single Windows path separators as shown. Replace the example with the real directory on your PC. Keep game files and derived data outside the repository. The local `victory.toml` is ignored by Git.

The configuration loader reads `.\victory.toml` from the current working directory by default. You can instead set `VICTORY_CONFIG` to the full path of another config file. Environment variables such as `VICTORY_GAME_DIR`, `VICTORY_CACHE_DIR`, and `VICTORY_DATA_DIR` take precedence over corresponding values in the TOML file.

You do **not** need to set `$env:VICTORY_GAME_DIR` in PowerShell when `[paths].game_dir` is already set in `victory.toml`. In particular, do not pass an unset environment variable to `Join-Path`.

## 2. Build diagnostics and check the configuration

```powershell
cargo build --profile diag -p victory --features diagnostics
if ($LASTEXITCODE -ne 0) {
    throw "VANGUARD diagnostics build failed."
}

.\target\diag\victory.exe config
```

The executable is `target\diag\victory.exe`. The config command prints the config file and the resolved `game_dir`, `cache_dir`, and `data_dir`. Fix any path marked as missing before launching.

A plain release build is a different executable and does not include diagnostics-only frame-operation tracing.

## 3. Optional frame capture setup

Do this **before** launching the game, in the same PowerShell session:

```powershell
$capture = Join-Path $env:USERPROFILE "Desktop\vanguard-capture"
New-Item -ItemType Directory -Force -Path $capture | Out-Null

$env:VICTORY_CAPTURE = $capture
$env:VICTORY_CAPTURE_WORLD = "1"
```

With diagnostics enabled, `VICTORY_CAPTURE_WORLD=1` selects the 2nd, 6th, 30th, and 80th frames counted from the first frame that records at least 500 draws. Add optional relative frame offsets with `VICTORY_CAPTURE_WORLD_ALSO`, for example:

```powershell
$env:VICTORY_CAPTURE_WORLD_ALSO = "118,119,120"
```

Those values are offsets from the first qualifying world frame; they are **not** offsets from a mission or boat-sequence start. Do not treat them as a boat capture schedule unless the relevant frame numbers have first been identified.

The capture folder must exist, the game must run, and selected frames must be reached before files appear. Operation traces are written as `frame-<number>-ops.txt` for selected frames. Frame images are saved according to the capture implementation's frame-selection rules.

## 4. Launch the title

For an experimental performance run, optionally set this in the same PowerShell
session **before** starting VANGUARD:

```powershell
$env:VICTORY_PERFORMANCE = "1"
```

This enables the asynchronous submission/relay and CPU-side batching preset
described in the README. It also reduces bindless texture upload work per
frame, so some textures may appear later while the queue drains. Leave it
unset for the baseline run. It is read when the GPU subsystem is initialized,
so changing it after launch has no effect.

The `run` subcommand requires the **full path to the actual title executable**. The configured `game_dir` does not automatically identify or select that executable. Do not guess its filename.

```powershell
$gameExe = (Read-Host "Paste the full path to the title executable").Trim().Trim('"')

if (-not [System.IO.Path]::IsPathRooted($gameExe)) {
    throw "Please enter an absolute path to the executable."
}
if (-not (Test-Path -LiteralPath $gameExe -PathType Leaf)) {
    throw "Executable not found: $gameExe"
}

$gameExe = (Resolve-Path -LiteralPath $gameExe).Path
& ".\target\diag\victory.exe" run $gameExe --execute --console series-x
```

Run this from the repository root. The same PowerShell session passes its environment variables to the launched process.

For a normal non-diagnostics build, run `cargo build --release -p victory` and substitute `target\release\victory.exe` in the launch command.

## 5. Inspect capture output

After the game has reached selected frames:

```powershell
Get-ChildItem -LiteralPath $env:VICTORY_CAPTURE -File |
    Sort-Object Name |
    Select-Object Name, Length
```

## Troubleshooting

- **`Join-Path` says its Path is null:** the referenced environment variable is not set. Use `victory.toml` as described above, or explicitly set the environment variable before using it.
- **`victory.exe config` says no game directory is configured:** check that `victory.toml` exists in the current working directory, that it contains `[paths].game_dir`, and that `VICTORY_GAME_DIR` is not overriding it.
- **The executable cannot be found:** `game_dir` is a directory, while `run` needs the full path to the title's executable file. Locate the correct executable in your own game installation and pass that exact path.
- **No operation trace files appear:** confirm that you launched `target\diag\victory.exe`, that `VICTORY_CAPTURE` points to an existing folder, that `diagnostics` is enabled, and that the game reached a frame selected by the capture settings.

Do not put game files, extracted assets, cache files, save data, or capture output inside the Git repository.
