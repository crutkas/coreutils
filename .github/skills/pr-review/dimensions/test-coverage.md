# Test coverage review

You are reviewing a PR diff for this `coreutils` fork and asking:
**are the changes adequately covered by tests?** Apply the shared output
contract in `_shared-contract.md`. Set `Domain: test-coverage` on every finding.

## Test surfaces in this repo

- **Integration tests:** `tests/by-util/test_<util>.rs`, one file per utility.
  These drive the built multicall binary via the `uutests` harness
  (`tests/uutests/`): `at_and_ucmd!()`, `new_ucmd!()`, `TestScenario`,
  `ucmd.arg(...).succeeds()/.fails()`, and assertions like
  `.stdout_is(...)`, `.stdout_only_fixture(...)`, `.stderr_contains(...)`,
  `.code_is(n)`, `.no_stderr()`. Fixtures live in
  `tests/fixtures/<util>/`.
- **Unit tests:** `#[cfg(test)] mod tests` inside `src/uu/<util>/src/*.rs` or
  `src/uucore/`.
- **GNU compatibility tests:** the upstream project also runs the real GNU test
  suite via `util/` + `GNUmakefile`; you generally can't run it here, but a
  behavior change may need a matching or new integration test that encodes the
  GNU expectation.
- **Running (for the recommendation, do not run the full suite):**
  `cargo test -p coreutils --no-default-features --features <util> --test tests -- test_<util>::<name>`.

## What to look for

- **New behavior, no test.** A bug fix or new option added to
  `src/uu/<util>/` should add or extend a test in `tests/by-util/test_<util>.rs`
  that fails without the fix and passes with it. For a Windows/platform fix,
  the test must actually run on that platform (not be `#[cfg]`-excluded from it).
- **Platform gating.** Watch `#[cfg(...)]` / `#[cfg(not(target_os = "windows"))]`
  guards on tests. A Windows fix whose test stays `not(windows)`-gated proves
  nothing. Conversely, a test newly enabled on a platform must genuinely pass
  there (and not depend on a Unix-only constant/helper that won't compile).
- **Regression intent.** Prefer a test that encodes the GNU-expected output /
  exit code, not just "it runs". Each test should make its intent obvious
  (what behavior/regression it guards), ideally with a short `// why` comment.
- **Edge cases from the correctness review.** If a null/empty/EOF/large-input
  or cross-platform concern was raised, is there a test for it? If not, that's
  a coverage finding too.
- **Harness misuse.** Env not preserved where the util needs it (the `ucmd`
  harness clears most env vars), reliance on machine state, temp files not
  cleaned up, tests that race on timing without `make_assertion_with_delay` or
  a retry, or tests that assert exact bytes where output is nondeterministic.
- **Fixtures.** New expected-output fixtures added under `tests/fixtures/<util>/`
  and referenced correctly; large binary fixtures avoided where a literal works.

## What to drop

- "Increase coverage to 100%" without a specific uncovered scenario.
- Unit tests for trivial getters / `Display` impls.
- Asking for tests on generated code.

## Severity guide for this dimension

- Bug fix / new option with zero tests → high.
- A platform fix whose test is excluded from that platform (so CI can't catch
  regressions) → high.
- New error path / edge case unreachable in current tests → medium.
- Test that pollutes machine state or races without a guard → medium (high if
  it will flake CI).
