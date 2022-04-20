# Pup 
[![BUILD](https://github.com/stevelauc/pup/workflows/Rust/badge.svg)](https://github.com/stevelauc/pup/actions/workflows/build.yml)

A command-line tool that automatically uploads images from the markdown document to
the GitHub repo and replaces the paths with the returned URL.

# Table of contents
   * [TO DO](https://github.com/SteveLauC/pup#to-do)
   * [Demo Video](https://github.com/SteveLauC/pup#demo-video)
   * [Getting Started](https://github.com/SteveLauC/pup#getting-started)
   * [What pup can NOT do](https://github.com/SteveLauC/pup#what-pup-can-not-do)
   * [Uninstallation](https://github.com/SteveLauC/pup#uninstallation)
   * [How it works](https://github.com/SteveLauC/pup#how-it-works)

# TO DO
- [x] Use system password management to store TOKEN
- [x] Multithreading file manipulation
- [x] Relative image path support

# Demo video
  ![demo](https://user-images.githubusercontent.com/96880612/163975456-fdebdee0-f68f-4227-8f11-b1c72cb4eaa3.gif)

# Getting started
1. Go to [token-settings](https://github.com/settings/tokens) to generate a new
   token, make sure it has access to your picture repo.

2. Execute the following script: 

   ```shell
   sudo bash -c "$(curl -fsSL https://raw.githubusercontent.com/stevelauc/pup/main/script/install.sh)"
   ```

3. Double check you have pup installed
   ```shell
   $ which pup
   /usr/local/bin/pup
   ```
4. init config file
   ```shell
   $ pup
   name is unset.
   repo is unset.
   mail is unset.
   ```
5. On your first run, `pup` will create a configuration file under
`$HOME/.config/pup` and complain about the incompleteness of the configuration
to you.

6. Then edit it to make it complete.
   ```shell
   $ vim $HOME/.config/pup/config.toml
   ```
   Make it something like the following one, remeber to replace each field with your own one.

   ```
   # configuration file for pup
   [user]
   github-user-name = "SteveLauC"
   github-repo-name = "test-repo"
   mail = "stevelauc@outlook.com"
   ```
7. Let's try it again!
   ```shell
   $ pup
   Please enter the TOKEN: 
   ```
   Ohhhh, since we haven't entered a token yet, pup asks us to do this.

8. All configuration is done! Time to enjoy:)
   ```shell
   $ pup your-markdown-file.md
   ```
   
# What pup can NOT do
1. Puting multi images in a single line is not supported.
2. The causes of failure may be confusing. For example, you may upload a photo 
   that has already been uploaded, perhaps you are expecting something like 
   `[FAILED]: DuplicateFile`, but pup will tell you `[FAILED]: ValidationFaile`.
   

# Uninstallation
   Execute the following script:

   ```shell
   sudo bash -c "$(curl -fsSL https://raw.githubusercontent.com/stevelauc/pup/main/script/uninstall.sh)"
   ```
# How it works
![workflow](https://github.com/SteveLauC/pic/blob/main/Page%201.jpeg)
