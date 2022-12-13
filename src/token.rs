//! Offers functionality of fetching, updating and deleting your secret TOKEN

use crate::echo::{echo_off, echo_on};
use keyring::{Entry, Result};
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

/// Fetch token from system password management.
///
/// if it is not set yet, ask the user to input then store it.
pub fn fetch_token() -> String {
    let pup = Entry::new("pup", "pup");

    if let Ok(token) = pup.get_password() {
        token
    } else {
        eprintln!("No TOKEN available.\nUse `pup --update-token` to set it.");
        exit(1);
    }
}

/// Update the TOKEN.
///
/// Old TOKEN will be overridden if it exists, or a new one will be set.
pub fn update_token() -> Result<()> {
    let pup = Entry::new("pup", "pup");
    print!("Please input the new TOKEN: ");
    stdout().flush().unwrap();
    echo_off(); // disable echo
    let mut new_token = String::with_capacity(20);
    stdin()
        .read_line(&mut new_token)
        .expect("expect a new token");
    // trim the "new line character"
    new_token.truncate(new_token.len() - 1);
    echo_on(); // enable echo
    pup.set_password(new_token.as_str())
}

/// Delete the existing TOKEN, if there is no TOKEN set, return.
pub fn delete_token() -> Result<()> {
    let pup = Entry::new("pup", "pup");
    if pup.get_password().is_ok() {
        pup.delete_password()
    } else {
        eprintln!("No token exists, nothing to delete.\nIf you wanna update it, use `pup --update-token` instead.");
        exit(1);
    }
}
