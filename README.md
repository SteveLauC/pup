### Pup [![BUILD](https://github.com/stevelauc/pup/workflows/Rust/badge.svg)](https://github.com/stevelauc/pup/actions/workflows/build.yml)
A command-line tool that automatically uploads images from the markdown document to
the GitHub repo and replaces the paths with the returned URL.

### TO DO
- [x] Use system password management to store TOKEN
- [x] Multithreading file manipulation
- [x] Support for symbolic link parsing
- [ ] Relative image path support

### Demo video
  ![demo](https://user-images.githubusercontent.com/96880612/163778336-a2fda462-0af0-45fa-afb5-bbec48b438fa.gif)

### Getting started
1. Go to [token-settings](https://github.com/settings/tokens) to generate a new
   token, make sure it has access to your picture repo.

2. Execute the following script: 

   ```shell
   $ curl https://raw.githubusercontent.com/stevelauc/pup/main/script/install.sh && sudo sh install.sh
   $ rm install.sh
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

   > Currently the token is stored in plain text, will fix it later!
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
### Uninstallation
   Execute the following script:

   ```shell
   $ curl https://raw.githubusercontent.com/stevelauc/pup/main/script/uninstall.sh && sudo sh install.sh
   $ rm uninstall.sh
   ```
### How it works
![workflow](https://github.com/SteveLauC/pic/blob/main/Page%201.png)
