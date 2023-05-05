//! Offers functionality of fetching, updating and deleting your secret TOKEN

use crate::echo::{echo_off, echo_on};
use anyhow::{anyhow, Result};
use keyring::{error::Error, Entry};
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

/// Fetch token from system password management.
///
/// if it is not set yet, ask the user to input then store it.
///
/// Used in `UserConfig::load()`.
pub fn fetch_token() -> String {
    let pup = Entry::new("pup", "pup").expect("expected it to be registered successfully");

    if matches!(pup.get_password(), Err(Error::NoEntry)) {
        eprintln!("No TOKEN available.\nUse `pup --update-token` to set it.");
        exit(1);
    } else {
        pup.get_password()
            .expect("TOKEN should be successfully fetched")
    }
}

/// Set the TOKEN.
///
/// If a TOKEN has already been set, an error will be returned.
pub fn set_token() -> Result<()> {
    let pup = Entry::new("pup", "pup")?;
    if !matches!(pup.get_password(), Err(Error::NoEntry)) {
        return Err(anyhow!("The TOKEN has already been set"));
    }

    print!("Please input the TOKEN: ");
    stdout().flush().unwrap();
    echo_off(); // disable echo
    let mut new_token = String::with_capacity(20);
    stdin()
        .read_line(&mut new_token)
        .expect("expect a new token");
    // trim the "new line character"
    new_token.truncate(new_token.len() - 1);
    echo_on(); // enable echo
    pup.set_password(new_token.as_str())?;

    Ok(())
}

/// Update the TOKEN.
///
/// If no token has been set, an error will be returned.
pub fn update_token() -> Result<()> {
    let pup = Entry::new("pup", "pup")?;
    if let Err(Error::NoEntry) = pup.get_password() {
        return Err(anyhow!(
            "No TOKEN has been set before, use `pup --set-token` to set it first."
        ));
    }

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
    pup.set_password(new_token.as_str())?;

    Ok(())
}

/// Delete the existing TOKEN, if there is no TOKEN set, return.
pub fn delete_token() -> Result<()> {
    let pup = Entry::new("pup", "pup")?;
    pup.delete_password()?;

    Ok(())
}
