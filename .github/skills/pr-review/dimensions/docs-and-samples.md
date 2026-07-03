# Docs & localization sync review

You are reviewing a PR diff for this `coreutils` fork and asking:
**do the docs, help text, and localization files reflect this change?**
Apply the shared output contract in `_shared-contract.md`. Set
`Domain: docs-and-samples` on every finding.

This dimension is mostly read-only research — use `explore` agent type if
available, otherwise standard file reads.

## Docs & localization surfaces

- **`src/uu/<util>/locales/*.ftl`** — Fluent localization files (`en-US.ftl`
  is the source of truth; `fr-FR.ftl` etc. are translations). User-facing
  strings — help text, error/status messages — are looked up here via the
  `uucore::translate!` macro (`translate!("key")` / `translate!("key", "arg" => x)`).
- **`src/uu/<util>/README.md`** — per-utility readme.
- **`docs/`** and top-level `*.md` (`README.md`, `CONTRIBUTING.md`,
  `DEVELOPMENT.md`) — project docs and the utility list.
- Inline `--help`/usage text if the utility hasn't been fully migrated to
  Fluent.

## What to look for

- **New/changed user-facing string not localized.** A new error, status, or
  help message added as a hardcoded English literal (`println!`, `format!`,
  `USimpleError::new("...")`) in a utility that otherwise uses `translate!`
  should be added as an `.ftl` key and referenced via `translate!` instead.
- **New `.ftl` key with no consumer, or a `translate!("key")` with no matching
  key** in `en-US.ftl` — this fails at runtime/build. Cross-check keys ↔ usages.
- **Translation drift.** A key added to `en-US.ftl` but the change also touches
  other locale files inconsistently — note missing keys in sibling `.ftl`
  files (usually acceptable to leave untranslated, but flag deletions/renames
  that orphan a translation).
- **New option/flag not documented.** A new CLI option missing from the
  utility's `--help` description (`.ftl` `-help` / `-about` keys) or its
  `README.md`.
- **Behavior change contradicting docs.** Changed default, output format, or
  flag meaning while the README / help text still describes the old behavior.
- **Utility list / feature drift.** A newly added or renamed utility not
  reflected in the top-level `README.md` utility list or `Cargo.toml`
  feature/members lists (coordinate with the packaging dimension).
- **GNU manual reference.** For a compatibility-driven change, a link/comment
  pointing at the relevant GNU manual section helps reviewers — note its
  absence only when the change is non-obvious.

## What to drop

- Grammar tweaks unrelated to the change.
- Asking to update docs for behavior that didn't change.
- Requiring translations of new strings into every locale (English source is
  enough; translations follow separately).

## Severity guide for this dimension

- `translate!` key with no `.ftl` entry (or vice versa) → high (runtime/build
  breakage).
- New user-visible option/message missing from help/`.ftl`/README → medium.
- Behavior change that contradicts existing docs/help → medium.
- Missing GNU-manual pointer / minor doc polish → low.
