//! Offers functionality of disabling and enabling ECHO on tty

use nix::sys::termios::{tcgetattr, tcsetattr, LocalFlags, SetArg, Termios};

/// Disable echo on tty configuration
pub fn echo_off() {
    let mut t: Termios = tcgetattr(0).unwrap();

    t.local_flags.remove(LocalFlags::ECHO);

    tcsetattr(0, SetArg::TCSANOW, &t).unwrap();
}

/// Enable echo on tty configuration
pub fn echo_on() {
    let mut t: Termios = tcgetattr(0).unwrap();

    t.local_flags.insert(LocalFlags::ECHO);

    tcsetattr(0, SetArg::TCSANOW, &t).unwrap();
}
