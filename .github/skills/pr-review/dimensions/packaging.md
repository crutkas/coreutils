# Build, deps & cross-platform impact review

You are reviewing a PR diff for this `coreutils` fork and asking:
**does this change affect the build, the dependency surface, platform coverage,
or release artifacts?** Apply the shared output contract in `_shared-contract.md`.
Set `Domain: packaging` on every finding.

## Build & distribution surfaces

This is a Cargo workspace that builds one multicall binary (`coreutils`) plus
per-utility crates (`uu_<util>`), gated by Cargo **features** (one per utility).

| Surface | Where | Notes |
|---------|-------|-------|
| Workspace | root `Cargo.toml` | `[workspace] members`, shared `[workspace.dependencies]`, feature list mapping each util to `uu_<util>` |
| Per-util crate | `src/uu/<util>/Cargo.toml` | its deps, `[target.'cfg(...)'.dependencies]`, `[[bin]]` |
| Shared lib | `src/uucore/Cargo.toml` | changes here ripple to every util |
| Lockfile | `Cargo.lock` | must stay consistent when deps change |
| Advisory/policy | `deny.toml` | `cargo-deny` (licenses, bans, advisories) runs in CI |
| Build scripts | `**/build.rs` | codegen / platform detection |
| CI | `.github/workflows/*` | fmt, clippy `-D warnings`, per-OS test matrix, `cargo-deny`, MSRV |
| GNU test harness | `util/`, `GNUmakefile` | runs the real GNU suite |

## What to look for

- **New dependency.** A new crate in a `Cargo.toml`: is it necessary, actively
  maintained, license-compatible (MIT/Apache — `deny.toml` enforces), and
  added to `[workspace.dependencies]` with a pinned version rather than ad-hoc?
  Flag heavy deps for a small feature, duplicate functionality already in
  `uucore`, or a dep that pulls a large transitive tree.
- **Version / lockfile drift.** `Cargo.toml` dependency changed but `Cargo.lock`
  not updated (or vice versa); a version bump that isn't needed.
- **Feature wiring.** A new utility or optional capability must be wired into
  the root `Cargo.toml` feature list and `members`, and the util must be
  reachable from the multicall dispatcher. Flag a new `uu_<util>` not added to
  the workspace feature map.
- **MSRV.** New syntax/std APIs that exceed the project's `rust-version`
  (`edition`/`rust-version` in `Cargo.toml`). Flag use of a std API newer than
  MSRV.
- **Cross-platform build.** `#[cfg(...)]` and `[target.'cfg(...)'.dependencies]`
  must keep every target compiling. A Windows-only dep or code path must not
  break the Unix build and vice versa; a `use`/import used on all platforms but
  only available on one is a build break. WASI (`target_os = "wasi"`) often
  needs stubs.
- **`build.rs` changes.** Codegen or platform detection that could fail on a
  target, or that reads the environment at build time.
- **CI impact.** Changes that would trip `cargo fmt --check`, `clippy
  -D warnings`, `cargo-deny`, or the per-OS matrix. A change that only compiles
  on the author's OS is a CI failure waiting to happen.
- **`unsafe`/FFI deps.** New `libc`/`windows-sys`/`rustix`/`nix` feature flags —
  ensure the minimal feature set is enabled.

## What to drop

- Suggesting a version bump unless the diff is clearly a release.
- Asking for new packaging artifacts outside the existing model.
- Bikeshedding dependency choice when the existing one is already in-tree.

## Severity guide for this dimension

- Change compiles on one OS but breaks another target's build → high.
- Use of a std/lang feature newer than MSRV → high.
- New utility/feature not wired into the workspace feature map / dispatcher →
  high.
- New dependency that duplicates `uucore` or has license/advisory issues →
  medium (high if `deny.toml` would reject it).
- `Cargo.toml`/`Cargo.lock` drift → medium.
