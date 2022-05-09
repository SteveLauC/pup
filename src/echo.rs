//! echo.rs: offers functionality of disabling and enabling ECHO on tty

use termios::{Termios, TCSANOW, tcsetattr, ECHO};

/// purpose: disable echo on tty configuration
/// 
/// action: unset ECHO bit of termios.c_lflag
pub fn echo_off() {
    let mut t = Termios::from_fd(0).unwrap();

    t.c_lflag &= !ECHO;

    tcsetattr(0, TCSANOW, &t).unwrap();
}

/// purpose: enable echo on tty configuration
/// 
/// action: set ECHO bit of termios.c_lflag
pub fn echo_on() {
    let mut t = Termios::from_fd(0).unwrap();

    t.c_lflag |= ECHO;

    tcsetattr(0, TCSANOW, &t).unwrap();
}