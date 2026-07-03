// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
use regex::Regex;
use std::env;
use uutests::new_ucmd;
use uutests::util::is_ci;

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails_with_code(1);
}

#[test]
fn test_normal() {
    let result = new_ucmd!().run();
    println!("env::var(CI).is_ok() = {}", env::var("CI").is_ok());

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
    if (is_ci() || uucore::os::is_wsl()) && result.stderr_str().contains("no login name") {
        // ToDO: investigate WSL failure
        // In the CI, some server are failing to return logname.
        // As seems to be a configuration issue, ignoring it
        return;
    }

    result.success();
    assert!(!result.stdout_str().trim().is_empty());
}

#[test]
fn test_help() {
    new_ucmd!()
        .arg("--help")
        .succeeds()
        .stdout_contains("Print user's login name");
}

#[test]
fn test_output_format() {
    let result = new_ucmd!().run();
    if (is_ci() || uucore::os::is_wsl()) && result.stderr_str().contains("no login name") {
        return;
    }
    result.success();
    assert!(
        Regex::new(r"^\w+\n$")
            .unwrap()
            .is_match(result.stdout_str()),
        "unexpected logname output: {:?}",
        result.stdout_str()
    );
}

// Regression test for #11056: on Windows, `logname` must print the current
// user's account name (via GetUserNameW) instead of failing with
// "logname: no login name" (exit code 1) as it did before Windows support
// was added.
#[test]
#[cfg(windows)]
fn test_windows_login_name() {
    let result = new_ucmd!().succeeds();
    let out = result.stdout_str().trim_end();
    assert!(!out.is_empty(), "logname printed an empty name");
    // GetUserNameW returns the same account name as the USERNAME environment
    // variable of the test process, so use it as the expected value when set.
    if let Ok(expected) = env::var("USERNAME") {
        if !expected.is_empty() {
            assert_eq!(out, expected);
        }
    }
}
