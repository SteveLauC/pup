//! offers functionality of fetching, updating and deleting your secret TOKEN

use crate::echo::{echo_off, echo_on};
use keyring::{Entry, Result};
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

/// purpose: fetch token from system passwrod mangaement
///          if it is not set yet, ask the user to input and then store it
///          
/// return: the TOKEN
pub fn fetch_token() -> String {
    let pup: Entry = Entry::new("pup", "pup");

    if let Ok(token) = pup.get_password() {
        token.trim().to_owned()
    } else {
        eprintln!("No TOKEN available.\nUse `pup --update-token` to set it.");
        exit(1);
    }
}

/// purpose: update token
///
/// action: Old TOKEN will be overridden if there was a old token; If there was
///         no token, set a new one.
///       
/// return: `Ok(())` on success, `keyring::error::Error` on error.
pub fn update_token() -> Result<()> {
    let pup: Entry = Entry::new("pup", "pup");
    print!("Please input the new TOKEN: ");
    stdout().flush().unwrap();
    echo_off(); // disable echo
    let mut new_token: String = String::with_capacity(20);
    stdin()
        .read_line(&mut new_token)
        .expect("expect a new token");
    echo_on(); // enable echo
    pup.set_password(new_token.as_str())
}

/// purpose: delete the existing TOKEN
///
/// action: delete the TOKEN if there was one
///
/// return: when there is a TOKEN, `Ok(())` on success, `keyring::error::Error`
///         on error.
///         Exit the whole program if no TOKEN available
pub fn delete_token() -> Result<()> {
    let pup: Entry = Entry::new("pup", "pup");
    if pup.get_password().is_ok() {
        pup.delete_password()
    } else {
        eprintln!("No token exists, nothing to delete.\nIf you wanna update it, use `pup --update-token` instead.");
        exit(1);
    }
}
