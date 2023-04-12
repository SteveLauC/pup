//! echo.rs: offers functionality of disabling and enabling ECHO on tty

use termios::{tcsetattr, Termios, ECHO, TCSANOW};

/// purpose: disable echo on tty configuration
///
/// action: unset ECHO bit of termios.c_lflag
pub fn echo_off() {
    let mut t = Termios::from_fd(0).expect("pup(tcgetattr): failed to get terminal configuration");

    t.c_lflag &= !ECHO;

    tcsetattr(0, TCSANOW, &t).expect("pup(tcsetattr): failed to turn off echo bit");
}

/// purpose: enable echo on tty configuration
///
/// action: set ECHO bit of termios.c_lflag
pub fn echo_on() {
    let mut t = Termios::from_fd(0).expect("pup(tcgetattr): failed to get terminal configuration");

    t.c_lflag |= ECHO;

    tcsetattr(0, TCSANOW, &t).expect("pup(tcsetattr): failed to turn on echo bit")
}
