# Developer skills

Skills in this directory are for **contributors working on this
`coreutils` fork** (a fork of [`uutils/coreutils`](https://github.com/uutils/coreutils)).
They are read by Copilot CLI (and other agents) to perform repo-specific
developer tasks like reviewing a change before it goes up as a PR.

> **Fork-local tooling — not for upstream.** These skills live only in this
> fork and are a personal review aid. Do **not** include `.github/skills/` in
> any pull request opened against `uutils/coreutils`.

## Available skills

| Skill | Purpose |
|-------|---------|
| [`pr-review/`](pr-review/SKILL.md) | Multi-dimensional review of a PR / feature-branch diff for a coreutils utility (correctness & GNU compatibility, cross-platform behavior, security, CLI/GNU-flag compatibility, alternative solutions, test coverage, docs & localization, build/deps impact, and a multi-model cross-check). Reports findings to stdout; does not apply fixes. |

## Conventions

- Each skill is a directory containing a `SKILL.md` (the entry point the
  orchestrating agent reads) and any supporting prompt fragments.
- Skills do not run scripts. The orchestrating agent uses its own tools
  (`task`, `grep`, `view`, `powershell`/`bash` for git, etc.) following the
  instructions in `SKILL.md`.
- Prompt fragments meant to be passed verbatim to sub-agents live under a
  `dimensions/` subfolder.
- Output goes to stdout unless the user explicitly asks for a file.
