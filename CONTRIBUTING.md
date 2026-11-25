# Contributing

Thanks for helping improve this community fork of [JoshuuDev/victory-amd](https://github.com/JoshuuDev/victory-amd).

This is experimental compatibility-layer work. A useful contribution is a small, reproducible investigation with clear evidence—not an unverified claim that a subsystem is fixed.

## Before you start

1. Read the [README](README.md), especially the upstream relationship, current status, and license notes.
2. Check [TEAM_TASKS.md](docs/TEAM_TASKS.md) and avoid duplicating active work.
3. Make sure you can build the project before changing code.
4. Keep game files, extracted assets, save data, shader caches, logs containing private paths, and local configuration out of Git.

## Branch and pull request workflow

- Start from the latest `team-integration` branch.
- Create a focused branch, for example `work/shader-opcode-name` or `work/descriptor-lifetime`.
- Keep each pull request limited to one topic.
- Open a pull request targeting `team-integration`, not the baseline `victory-amd` branch.
- Explain what changed, why it is needed, how it was tested, and what remains unknown.
- Do not force-push shared branches or rewrite history others may be using.

## What to include in a bug report

- Exact commit SHA and branch.
- Windows version, CPU, GPU, driver version, and game profile where relevant.
- Exact reproduction steps and what you expected versus what happened.
- Relevant log excerpts, shader hashes, or captures.
- Whether the issue reproduces on a clean run.
- Any environment variables or non-default settings used.

Remove usernames, personal file paths, tokens, account details, and other private information before posting logs.

## Engineering expectations

- Preserve upstream attribution, license headers, and project history.
- Prefer the smallest change that tests a clear hypothesis.
- Do not describe a suspected cause as confirmed without evidence.
- Run relevant tests and formatting checks; report commands and results accurately.
- Do not include copyrighted game code, game assets, dumps, or proprietary files in commits or issues.

## Review checklist

- [ ] The change has one clear purpose.
- [ ] The PR describes the evidence and test setup.
- [ ] Tests/checks and their results are listed.
- [ ] No game data, secrets, or personal information are included.
- [ ] Upstream credits and license requirements are preserved.
