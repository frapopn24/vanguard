# Victory AMD — Community Development Fork

[![Based on JoshuuDev/victory-amd](https://img.shields.io/badge/base-JoshuuDev%2Fvictory--amd-2563eb?logo=github&logoColor=white)](https://github.com/JoshuuDev/victory-amd)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Status: experimental](https://img.shields.io/badge/status-experimental-orange.svg)](#current-status)

**An independent community fork of [JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd), focused on graphics compatibility, resource handling, performance profiling, and gameplay correctness.**

This repository is not the original project and is not an official release. It exists to coordinate experiments and improvements on top of JoshuuDev's AMD-focused Victory fork. Please visit and credit the upstream project: **[github.com/JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd)**.

> [!IMPORTANT]
> This is experimental compatibility-layer software. Results vary by GPU, driver, game build, and configuration. Rendering progress does not mean the game is fully playable. Expect missing features, low frame rates, crashes, and incomplete gameplay behavior.

## Current status

The latest locally reported test on the maintainer's system reached **Chapter 1**. Character faces and most observed textures were rendering correctly, with roughly **3–5 FPS** in that session. The boat sequence currently has a water/vehicle interaction failure that causes the mission to end. The cause has **not** been established; it may involve missing GPU work, game-side compatibility, or another subsystem.

These are observations from one experimental setup, not independently reproduced results or a claim of general compatibility. See [the team workstreams](docs/TEAM_TASKS.md) for the current investigation areas.

## What this fork is working on

- **Shader translation:** support for additional shader instructions and pipeline creation failures.
- **Resource handling:** descriptor heaps, texture views, streaming, and memory correctness.
- **Performance:** CPU submission overhead, GPU workload, synchronization, and memory transfers.
- **Gameplay compatibility:** investigate water, vehicle, collision, and other mission-critical behavior.

## Getting started

This project builds from source on Windows. Use your own legally obtained game files. Keep game data, extracted assets, shader caches, save data, and the local `victory.toml` configuration outside Git; the local config file is ignored by Git.

For a fresh clone:

```powershell
git clone https://github.com/frapopn24/vanguard.git
cd vanguard

# Create the local config once, then edit [paths].game_dir.
Copy-Item .\victory.example.toml .\victory.toml
notepad .\victory.toml

cargo test --workspace
cargo build --profile diag -p victory --features diagnostics

# Check the resolved paths before launching.
.\target\diag\victory.exe config
```

Set `[paths].game_dir` in `victory.toml` to your game directory. The example file contains a placeholder path; replace it with the real location. Environment variables such as `VICTORY_GAME_DIR` override values in the file, so leave them unset unless you intentionally want an override.

**Important:** `victory config` checks the configured directories, but `victory run` still requires the full path to the title's actual executable as its argument. It does not guess the executable filename from `game_dir`. See [docs/STARTUP.md](docs/STARTUP.md) for the complete PowerShell launch and diagnostics-capture instructions.

The commands above are a starting point, not a guarantee that every environment is ready to run the title. Review the requirements and asset-policy notes below before launching.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and choose a focused task from [docs/TEAM_TASKS.md](docs/TEAM_TASKS.md). Use feature branches and submit pull requests into `team-integration`; do not push experimental changes directly to the baseline branch.

## Project links

- **Upstream base:** [JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd)
- **Original VIctory project:** [Marvisak/VIctory](https://github.com/Marvisak/VIctory)
- **Team workstreams:** [docs/TEAM_TASKS.md](docs/TEAM_TASKS.md)
- **Windows startup and diagnostics:** [docs/STARTUP.md](docs/STARTUP.md)
- **Shader and pipeline preload manager:** [docs/PRELOAD_MANAGER.md](docs/PRELOAD_MANAGER.md)
- **Contribution guide:** [CONTRIBUTING.md](CONTRIBUTING.md)

## Credits and relationship to upstream

This repository is an independent fork. It builds on work by JoshuuDev and the contributors to the projects in its technical lineage. Keep upstream notices, license terms, and attribution intact when modifying or redistributing code. This project is not affiliated with or endorsed by the game's developer or publisher.

---

## Inherited DirectStorage change

The inherited branch documentation records a DirectStorage read-path change: kernel reads into title memory are first received into a buffer and then copied into title memory from user mode. The stated motivation was to avoid a guarded-page interaction that had caused `ERROR_INVALID_USER_BUFFER` (`0x800706f8`) during raw DirectStorage reads. The same note reports a 5.5-minute run without a failed request on one test machine.

This is a historical, single-machine result—not a guarantee that DirectStorage is fully correct or that the issue is resolved on every system. The original inherited note identifies [myokex/victory-amd](https://github.com/myokex/victory-amd) and commit `070df42` in its provenance. This community repository separately identifies [JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd) as its stated project base; retain both references when discussing the inherited history.

## Technical lineage and implementation notes

The current community codebase is based on [JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd). Its wider technical lineage includes [victory-fork](https://github.com/fxceless/victory-fork) and the original [VIctory](https://github.com/Marvisak/VIctory). This repository keeps those upstream origins and contributor credits visible. Historical hardware notes below describe earlier upstream testing, not a guarantee for every system.

State on that machine (2026-11-25, Series X profile): the prologue's cutscene and
the first room are drawn, with the player moving, at 2 to 3 frames per second.
Runs of 8 minutes mostly end without a device loss; one ended with
`DXGI_ERROR_DEVICE_HUNG` after about 5 and a half minutes, cause unknown. Lamps
were dark in one run. The first room's walls were missing in several runs; with the
guard over all the views of a page (below) they were there in two runs out of two,
too few to call it fixed. Some shots of the opening cutscene come out white.
Ray-traced light does not run.

What it changes over `victory-fork` (each is explained in a comment where it is):

- Texture views are told apart by the memory their mips really take, so textures
  streamed in part are not taken for one another and are uploaded from their first
  mip that is there.
- Views of placed textures the host lost sight of are put back
  (`VICTORY_TEXTURE_SWEEP`).
- Dispatches of more than 65,535 groups are made in two parts (the final gather at
  2560x1440 asks for 115,200).
- DirectStorage requests read their page array when they run, and wait for it.
- The list of pipelines to make early keeps what earlier runs listed, and compute
  pipelines get their own waiting time while a world loads.
- A write of the title's through one view of a physical page is seen by the copies
  taken through its other views (`VICTORY_VIEW_GUARD=0` turns it off).
- Textures picked by index are looked at again after their upload
  (`VICTORY_HEAP_RECHECK=0` turns it off).
- View indices past the host heap are clamped on AMD too.
- Programs that stop the AMD driver's compiler, or take it minutes and tens of GB,
  are left out (ray-tracing resolves).
- Diagnostics: `VICTORY_NULL_MAGENTA`, `victory warm`, more captures.

The guard over all the views of a page is code of
[victory-fork #115](https://github.com/fxceless/victory-fork/pull/115) by Spexivus
with [its correction](https://github.com/Spexivus/victory-fork/pull/1) by joroyt and
a part of [VIctory #1122](https://github.com/Marvisak/VIctory/pull/1122) by tightfall,
adapted here. The second look at textures is fxceless's idea (victory-fork `a803c87`),
written anew. Other parts were prompted by pull requests and issues of VIctory and
victory-fork, and their code was written anew. Developed with [Claude Code](https://claude.com/claude-code).

A work-in-progress fork of [VIctory](https://github.com/Marvisak/VIctory), a
compatibility layer that runs an Xbox Series title natively on Windows from
files the user supplies. This fork follows upstream
`main` and adds fixes aimed at one goal: getting the game's world to render
correctly and playably on one particular PC.

**This repository contains no game files, code or assets.** You need your own
copy of the game. It is not affiliated with or endorsed by the game's
developer or publisher.

## Credits

- **[VIctory](https://github.com/Marvisak/VIctory)** by Marvisak and its
  contributors is the base of everything here: the loader, the runtime
  services, the Direct3D 12 back end and the shader recompiler. Licensed
  GPL-3.0-or-later; so is this fork. Upstream's history is kept intact below
  the single commit that carries this fork's changes.
- **[XCL](https://github.com/ivall/xcl)** by ivall and its contributors
  (GPL-2.0-only) was used as a reference for how the same title behaves under
  another layer. Fixes inspired by it were written anew in VIctory's code.
- This fork's changes were developed with [Claude Code](https://claude.com/claude-code).

## Status of this fork (2026-11-25)

**Not playable yet: 2.5 to 4 frames per second, and shadowed areas are black.**
Tested on one machine: Windows 11, Intel i7-14700KF, NVIDIA RTX 4070 12 GB,
32 GB RAM. The Series X profile is used throughout.

| Stage | State in this fork |
|-------|--------------------|
| Boot, menus, start screen | works, 30 fps (as upstream) |
| Opening cutscene | plays from start to finish, in order, lit: title cards, credits cards, night and dusk shots |
| Saves | the game autosaves a checkpoint and loads it again |
| First gameplay scene (the prologue) | renders: characters, textures, sky, sunlit exterior; HUD, pause menu and pad input work |
| Light in shadow | missing: the game's ray-traced bounce light and ray-traced lamp and sun shadows do not run yet, so anything the sun does not hit directly is black |
| Frame rate in the world | 2-4 fps: 420-450 ms per frame by default, 240-350 ms with the newer switches (`VICTORY_RELAY=2`, or `VICTORY_LATE_WAIT=3 VICTORY_WIDE_BACK=1 VICTORY_CARD_BUFFERS_MIN_MB=1 VICTORY_CARD_BUFFERS_MB=1024`); it started at 3-6 seconds per frame |
| Stability | no driver hang or device loss in normal runs (a hang introduced by the wave64 update was found and fixed: its barrier form for 64-lane programs is now off unless `VICTORY_WAVE_WIDE=1`) |

### What this fork changed (summary)

Rendering:
- Meshlet culling read its scene buffers as the wrong kind of view, so the world drew almost nothing; fixed.
- Per-draw descriptor tables could run out, after which draws used slots never made for them and the card hung; fixed.
- Light packed by the game's compute shaders was read back through empty copies (sky, probes, fog); fixed for every mip level and layer.
- The fog reconstruction, shadow-mask, hair, blend-shape skinning, fire and several material programs did not translate; fixed.
- Gathers at a named mip level (with and without an offset) read level 0; fixed.
- Implicit-level samples outside pixel shaders produced invalid DXIL in 730 compute programs; fixed.
- Rectangle lists, depth and stencil twins for depth textures written by compute, texture read-backs.
- Ray tracing, in progress and off by default: the game's own structure-builder programs translate, title memory can be read by raw address from shaders, the hardware ray/box and ray/triangle test is emulated in DXIL (checked against a reference on the GPU), and 59 of the 64 trace programs translate. Nothing is lit by it yet. See [docs/fork-ray-tracing.md](docs/fork-ray-tracing.md).

Speed and memory:
- Waits for memory values, write-watch queries and page queries no longer walk page tables per call.
- Large title buffers get a copy in card memory, with shader writes found by hashing on the card.
- Submissions can be sent ahead of the wait for the card (`VICTORY_LATE_WAIT`), with a relay thread that lands results as the card finishes (`VICTORY_RELAY`): in a first five-minute run the card's idle time behind the submitting thread fell from about 136 to 30 ms per frame.
- Title memory can be opened as a graphics heap so that buffers are used in place instead of copied (`VICTORY_INPLACE`, first stage only; a probe confirmed the approach on the test GPU).
- The bindless texture queue no longer uploads stale views; the view table no longer grows without bound; the pipeline library is mapped instead of read into memory.
- An intermittent device loss was traced to the pipeline library file being replaced while mapped; fixed.

- Translated shaders get clean-up passes over the finished module (dead code, constant folding, reuse of values, branch facts); the pack's host instruction count falls by about 43 %, worth about 20 ms of GPU time per frame on the test machine (`VICTORY_SHADER_OPT=0` turns them off).

Most of the newer speed changes sit behind environment switches that are off
by default (`VICTORY_LATE_WAIT`, `VICTORY_RELAY`, `VICTORY_WIDE_BACK`,
`VICTORY_CARD_BUFFERS_MIN_MB`, `VICTORY_INPLACE`, `VICTORY_BVH_BUILDER`, ...).
Each is described in a comment where it is read.

For an experimental gameplay performance preset, set this **before** launching:

```powershell
$env:VICTORY_PERFORMANCE = "1"
```

The preset enables asynchronous submission/relay (late wait stage 3 with relay
stage 2), parallel shader-write snapshots, comparisons and writeback merges,
batches adjacent non-overlapping title-memory copies, combines repeated card
memory hash passes, and uses a 1 MiB threshold for putting large title-buffer
copies in card memory. The card-memory budget is raised
from 512 MiB to at most 1 GiB; allocation still respects the live DXGI video
memory budget and can be reduced with `VICTORY_CARD_BUFFERS_MB`.

To protect frame time, the preset also avoids blocking texture refreshes while
the title remaps memory and limits bindless texture uploads to 48 MiB / 60 ms
per frame (96 MiB / 120 ms while catching up), rather than 192 MiB / 250 ms
(512 MiB / 1.5 s while catching up). This is an intentional visual trade-off:
some dynamically indexed textures can appear later while their upload queue
drains. Override these limits with `VICTORY_HEAP_TEXTURES_MB`,
`VICTORY_HEAP_TEXTURES_MS`, `VICTORY_HEAP_TEXTURES_CATCHUP_MB` and
`VICTORY_HEAP_TEXTURES_CATCHUP_MS`; set `VICTORY_TEXTURE_NO_WAIT=0` to restore
blocking texture refreshes.

Each corresponding `VICTORY_*` setting can override the preset explicitly;
for example, set `VICTORY_WIDE_BACK=0` to turn off only that part. This preset
is experimental: compare it with the default path on the same scene and watch
for rendering errors, hangs, or device removal. For an even more aggressive
diagnostic trade-off, `VICTORY_HEAP_TEXTURES=0` disables the preparation pass
for dynamically indexed textures entirely; expect many such materials to be
missing. This is not part of the default preset. The preset does not guarantee
higher FPS on every adapter. Do not combine it with `VICTORY_GPU_TIMING`, which
deliberately disables asynchronous submission.

### Known problems

- Everything in shadow is black (see above).
- The interior of the first gameplay room is sometimes not drawn after the cutscene hands over to gameplay.
- Around 24 GB of memory is in use during play.
- A few style lints are switched off for this fork in the workspace `Cargo.toml` until the newer code is tidied.
- Many changes here are specific to this title and this GPU, and some are marked `shortcut:` in the code where they approximate what the console does.

The upstream VIctory README material below is retained for reference; its
status section describes upstream, not this fork. The PowerShell launch example
is adapted here to require the title executable's explicit path.

---

# VIctory

Runs the title (Xbox Series X|S) natively on Windows, from files
the user supplies.

The title's x86-64 code runs unchanged inside our process; VIctory supplies
the platform underneath it and converts the title's data, ahead of time where
it can and on first use where it must (shaders). See
[docs/architecture.md](docs/architecture.md).

## Status (2026-11-23)

**Not playable yet.** The title boots, runs its menus at its own 30 fps and
loads the world, where rendering fails.

| Stage | State |
|-------|-------|
| Boot, first-boot screens (accessibility, language, settings, legal) | works, 30 fps |
| Intro video with sound | works |
| Start screen, profile and saves | works |
| Loading into the world (about 50 s) | works |
| The world (first mission's intro) | reached and runs with indirect draws; its title card renders, the scene after it is still dark; 0.5-1.8 s per frame |

Tested on one machine: Windows 11, AMD RX 6700 XT.

> **Warning.** Entering the world on a real GPU can currently hang it, and
> Windows then resets the display driver (every monitor goes black for a
> moment). To look further, use a diagnostics build with
> `VICTORY_D3D12_WARP=1`, which renders on the software adapter.

What each part does today and what it still needs:

| Subsystem | Today | Needs |
|-----------|-------|-------|
| Loader | Runs the title's executable and its DLLs in our process; crash reports name the faulting code and are kept in `crashes.log` | – |
| Runtime services | One signed-in offline user, licences, packages, saves that survive a kill at any step, text entry dialog | Speech is silent; the dialog and relaunch are untested live |
| Network | Offline by design: the title cannot reach the network | – |
| Files and streaming | Archives and streamed data read, zlib and BCPack decoded in parallel | Loading is limited by writes into title memory |
| Input | XInput pads, keyboard, keyboard stand-in pad | Trigger rumble |
| Audio | Menu, video and loading sound; XMA voices and Opus streams decode | Reverb is silent; Opus results unverified against the title |
| GPU: menus | Draws, compute and presentation on host D3D12; only changed title memory is copied | Frame pacing drifts on one screen in release builds |
| GPU: world | Mesh pipelines, shader-written buffers, descriptor lifetimes, indirect execution (arguments clamped), pipelines made by worker threads and kept on disk | Most of the scene is still missing (untranslated shaders drop draws); frames take seconds; real queries, depth bounds, HDR and scaling are missing |
| Shaders | Console GPU code recompiled to DXIL at first use, cached on disk. Menus: all. World: mesh 8,191 of 8,506, vertex/pixel 10,617 of 12,346, compute 1,643 of 2,539 | 64-bit image atomics, merged geometry stages, the ray-tracing builder's shaders; some compute programs need a GPU with 64-lane waves |
| Textures | Console tiled layouts and detiling for the modes the title uses | Render-target metadata planes |
| Series S mode | Runs through the menus to the loading screen | The world, as above |

## Getting started

Requirements: Windows 10/11 x64, a CPU with AVX2, a Direct3D 12 GPU with
shader model 6.6 (and mesh shaders for the world), Rust (pinned in
`rust-toolchain.toml`).

The title also needs several GiB of available Windows commit at startup.
Check **Committed** under **Task Manager → Performance → Memory**. If
`crashes.log` reports Windows error 1455, close other applications or increase
the Windows paging file size, then retry.

```powershell
cargo xtask install-hooks                # asset-policy check on every commit
copy victory.example.toml victory.toml   # then set your paths (outside the repo)
cargo test --workspace                   # needs no game data
cargo run -p victory -- config           # check your paths
```

Run the title from the repository root. The executable argument must be the full path to the title's actual executable; do not substitute the game directory itself or assume a particular executable filename:

```powershell
cargo build --release -p victory
$gameExe = (Read-Host "Full path to the title executable").Trim().Trim('"')
if (-not [System.IO.Path]::IsPathRooted($gameExe)) {
    throw "Please enter an absolute path to the executable."
}
if (-not (Test-Path -LiteralPath $gameExe -PathType Leaf)) {
    throw "Executable not found: $gameExe"
}
$gameExe = (Resolve-Path -LiteralPath $gameExe).Path
& ".\target\release\victory.exe" run $gameExe --execute --console series-x
```

For the diagnostics build and frame-operation capture, see [docs/STARTUP.md](docs/STARTUP.md).

The window caption shows the framerate. Saves, settings and `crashes.log`
are kept under the data folder (default `%LOCALAPPDATA%\victory\data`), derived
data under the cache folder.

Input: XInput pads. With none connected the keyboard is a pad while the
title's window has focus:

| Keys | Pad |
|------|-----|
| W A S D / I J K L | left / right stick |
| arrows | D-pad |
| Enter, Backspace, X, Y | A, B, X, Y |
| Q / E, Z / C | shoulders, triggers |
| F / H | stick clicks |
| Esc, Tab | Menu, View |

## Commands

```
victory inspect <exe|dll> [--imports] [--exports]
victory census [paths...] [--missing] [--strict]
victory lib-exports <import.lib> [--list]
victory shader-info [paths...] [--verbose] [--disasm]
victory translate [paths...] [--validate] [--pipelines] [--list] [--jobs N]
victory preload [paths...] [--jobs N] [--no-compute-pipelines] [--status] [--report PATH] [--strict]
victory warm [paths...]   # alias for preload
victory run <exe> [--execute] [--unimplemented trap|zero] [--strict]
            [--console series-x|series-s] [-- <title arguments>]
victory config
```

Logging: `VICTORY_LOG=victory_loader=trace,info` (a release build logs
info and above).

## Debugging

A release build has no debugging aids. Build them in with

```powershell
cargo build --profile diag -p victory --features diagnostics   # target\diag\victory.exe
```

| Variable | Effect |
|----------|--------|
| `VICTORY_LOG=<target>=debug,...` | debug and trace output |
| `VICTORY_STACKS=20,60` | every thread's stack at those seconds |
| `VICTORY_WATCH=<rva>,...` | logs a title u32 when it changes |
| `VICTORY_KEYS=30:Enter,35:X` | presses keys at those seconds, focus or not |
| `VICTORY_CRASH_SAVE=<rva>:<len>,...` | saves title memory to the cache folder on a crash |
| `VICTORY_CAPTURE=<dir>` | saves presented frames (powers of two, or `VICTORY_CAPTURE_FRAMES=<list>`) |
| `VICTORY_DUMP_TARGETS=1` | with the two above, saves every render target of those frames |
| `VICTORY_D3D12_DEBUG=1` | host debug layer |
| `VICTORY_D3D12_GBV=1` | with the above, GPU-based validation (bad descriptor use reported, not faulted; slow) |
| `VICTORY_D3D12_DRED=1` | on a GPU failure, the allocation hit and the commands in flight |
| `VICTORY_PIPELINE_WAIT_MS=24` | reduce the per-frame wait for background pipeline creation from the default of 500 ms. A draw can be omitted until its pipeline is ready |
| `VICTORY_SKIP_PIPELINES=<hash>,...` | omit draws and dispatches, including indirect ones, whose pipeline contains a listed 16-digit shader hash |
| `VICTORY_GPU_TIMING=1` | with the above, each list of a submission run and timed on its own; slow lists logged with their slowest commands |
| `VICTORY_D3D12_WARP=1` | host device on the software adapter (slow; cannot reset the driver) |
| `VICTORY_SHADER_LOOP_LIMIT=<n>` | translated shader loops stop after `n` iterations per lane instead of 2^20 (own shader cache entries); if a lower `n` makes world frames much faster, loops hitting the limit are what makes them slow |
| `VICTORY_AUDIO_CAPTURE=<file.wav>` | records what the title plays (our process only) |
| `VICTORY_XMA_DUMP=<dir>` | saves the first 32 XMA voices' context and input |

Every build appends crashes and host GPU failures to `crashes.log` in the
title's data folder.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
