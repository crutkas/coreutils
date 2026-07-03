# Alternative-solution review

You are reviewing a PR diff for this `coreutils` fork and asking:
**is there a simpler, more idiomatic, or already-existing way to do this in
this codebase?** Apply the shared output contract in `_shared-contract.md`.
Set `Domain: alternative-solution` on every finding.

## Repo-specific patterns to enforce

- **Reuse `uucore`.** `src/uucore/` is the shared library every utility builds
  on. Before hand-rolling, check whether `uucore` already provides it:
  - `uucore::fs` — path canonicalization, `is_symlink`, safe traversal helpers.
  - `uucore::error` — `UResult`, `UError`, `USimpleError`, `UUsageError`,
    `set_exit_code`, `FromIo` for mapping `io::Error`.
  - `uucore::display` / `Quotable` (`.quote()`) — GNU-style filename quoting.
  - `uucore::format` — printf-style and number formatting.
  - `uucore::parser` (size, mode, ranges), `uucore::translate!` (Fluent i18n),
    `show_error!` / `show_warning!` macros.
  New code that re-implements quoting, size parsing, error mapping, or path
  logic inline should call the helper instead — the inline version usually
  reintroduces an edge-case bug the helper already handles.
- **Platform abstraction.** Utilities put OS-specific code behind a
  `platform/{unix,windows}.rs` module or `#[cfg(...)]`. New OS-specific logic
  dumped inline in the main file (instead of the existing platform module)
  should be flagged.
- **Idiomatic Rust.** Prefer iterators/`?`/pattern-matching over manual index
  loops and nested `match` on `Option`/`Result`; prefer `if let`/`let else`
  over `.unwrap()`. Flag needless allocation (`to_string()`/`clone()` in hot
  loops, `collect()` then iterate once), and needless `String` where `OsStr`/
  `&[u8]`/`Cow` is correct (filenames!).
- **Don't fight the crate.** `clap` for args, `notify` for file watching, etc.
  Re-implementing what a workspace dependency already provides is a finding.
- **Premature abstraction.** A new trait/generic/module with a single caller
  and no anticipated second → recommend inlining.

## Cross-cutting checks

- Does this change duplicate logic that already exists in another utility or in
  `uucore`? Recommend extracting to / reusing `uucore`.
- Could a new function be a thin wrapper over an existing helper? If so,
  recommend the wrapper.
- Is the fix in the right layer? A cross-cutting fix belongs in `uucore` (one
  place, all utils benefit) rather than copy-pasted into one utility — but a
  `uucore` change has a large blast radius, so weigh both.

## What to drop

- Generic "this could be more functional" without a concrete callable
  alternative in the repo.
- Refactor suggestions that exceed the PR's scope ("rewrite this utility") —
  note them only as `low` with a tight recommendation, or skip.

## Severity guide for this dimension

- Re-implementing existing `uucore` logic (quoting, size/mode parsing, error
  mapping, path canonicalization) in a way that risks a bug → medium.
- OS-specific code placed inline instead of the existing `platform/` module →
  medium.
- Needless allocation/clone in a hot path with concrete cost → low/medium.
- Minor "could reuse helper X" with marginal benefit → low.
