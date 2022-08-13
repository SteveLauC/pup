//! echo.rs: offers functionality of disabling and enabling ECHO on tty

use nix::sys::termios::{tcgetattr, tcsetattr, LocalFlags, SetArg, Termios};

/// purpose: disable echo on tty configuration
///
/// action: unset ECHO bit of termios.c_lflag
pub fn echo_off() {
    let mut t: Termios = tcgetattr(0).unwrap();

    t.local_flags.remove(LocalFlags::ECHO);

    tcsetattr(0, SetArg::TCSANOW, &t).unwrap();
}

/// purpose: enable echo on tty configuration
///
/// action: set ECHO bit of termios.c_lflag
pub fn echo_on() {
    let mut t: Termios = tcgetattr(0).unwrap();

    t.local_flags.insert(LocalFlags::ECHO);

    tcsetattr(0, SetArg::TCSANOW, &t).unwrap();
}
