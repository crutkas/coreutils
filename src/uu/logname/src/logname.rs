// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) getlogin userlogin

use clap::Command;
use std::io::{Write, stdout};
use uucore::translate;
use uucore::{error::UResult, show_error};

#[cfg(unix)]
fn get_userlogin() -> Option<String> {
    use std::ffi::CStr;
    let login_ptr = unsafe { libc::getlogin() };
    if login_ptr.is_null() {
        None
    } else {
        Some(String::from_utf8_lossy(unsafe { CStr::from_ptr(login_ptr) }.to_bytes()).to_string())
    }
}

// Windows has no POSIX `getlogin`; the login name of the current user is best
// approximated by their account name, retrieved via `GetUserNameW` (the same
// approach `whoami` uses on Windows).
#[cfg(windows)]
fn get_userlogin() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::NetworkManagement::NetManagement::UNLEN;
    use windows_sys::Win32::System::WindowsProgramming::GetUserNameW;

    const BUF_LEN: u32 = UNLEN + 1;
    let mut buffer = [0u16; BUF_LEN as usize];
    let mut len = BUF_LEN;
    // SAFETY: `buffer` holds `BUF_LEN` elements and `len` is initialized to it;
    // on success `GetUserNameW` sets `len` to the char count including the NUL.
    if unsafe { GetUserNameW(buffer.as_mut_ptr(), &raw mut len) } == 0 {
        return None;
    }
    Some(
        OsString::from_wide(&buffer[..len as usize - 1])
            .to_string_lossy()
            .into_owned(),
    )
}

#[cfg(not(any(unix, windows)))]
fn get_userlogin() -> Option<String> {
    None
}

#[uucore::main(no_signals)]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let _ = uucore::clap_localization::handle_clap_result(uu_app(), args)?;

    if let Some(userlogin) = get_userlogin() {
        writeln!(stdout(), "{userlogin}")?;
        Ok(())
    } else {
        show_error!("{}", translate!("logname-error-no-login-name"));
        Err(1.into())
    }
}

pub fn uu_app() -> Command {
    Command::new("logname")
        .version(uucore::crate_version!())
        .help_template(uucore::localized_help_template("logname"))
        .override_usage(translate!("logname-usage"))
        .about(translate!("logname-about"))
        .infer_long_args(true)
}
