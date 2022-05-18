# Pup 
[![BUILD](https://github.com/stevelauc/pup/workflows/Rust/badge.svg)](https://github.com/stevelauc/pup/actions/workflows/build.yml)

A command-line tool that automatically uploads images from the markdown document to
the GitHub repo and replaces the paths with the returned URL.

# Table of contents
   * [TO DO](https://github.com/SteveLauC/pup#to-do)
   * [Supported platforms](https://github.com/SteveLauC/pup#supported-platforms)
   * [Demo Video](https://github.com/SteveLauC/pup#demo-video)
   * [Getting Started](https://github.com/SteveLauC/pup#getting-started)
   * [What pup can NOT do](https://github.com/SteveLauC/pup#what-pup-can-not-do)
   * [Uninstallation](https://github.com/SteveLauC/pup#uninstallation)
   * [How it works](https://github.com/SteveLauC/pup#how-it-works)

# TO DO
- [x] Use system password management to store TOKEN
- [x] Multithreading file manipulation
- [x] Relative image path support

# Supported platforms

   |Platform   |Supported                       |
   |-----------|--------------------------------|
   |Linux-amd64| Yes(prebuilt-binary-available) |
   |Linux-arm64| No                             |
   |macOS-amd64| Yes(need to build from src)    |
   |macOS-arm64| Yes(need to build from src)    |


# Demo video
  ![demo](https://user-images.githubusercontent.com/96880612/163975456-fdebdee0-f68f-4227-8f11-b1c72cb4eaa3.gif)

# Getting started
1. Go to [token-settings](https://github.com/settings/tokens) to generate a new
   token, make sure it has access to your picture repo.

2. get the `pup` binary

   * Linux(amd64)

       execute the following script: 

        ```shell
       sudo bash -c "$(curl -fsSL https://raw.githubusercontent.com/stevelauc/pup/main/script/install.sh)"
       ```
       If you are in a place where you need proxy to access GitHub(i.e. China Mainland)
       , and you have `https_proxy` set in your personal configuration, use the following 
       script:

       ```shell
       sudo -E bash -c "$(curl -fsSL https://raw.githubusercontent.com/stevelauc/pup/main/script/install.sh)"
       ```

    * macOS(Apple Silicon and Intel Mac):
        You need to build it from source

        ```shell
        $ git clone https://github.com/SteveLauC/pup.git && cd pup
        $ cargo build --release
        $ sudo cp target/release/pup /usr/local/bin
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

5. On your first run, `pup` will create a configuration file and complain about 
   the incompleteness of it to you. If you have `$XDG_CONFIG_HOME` validly set, 
   then it is located in `$XDG_CONFIG_HOME/pup`; Otherwise, it is under 
   `$HOME/.config/pup`

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
   No TOKEN available.
   Use `pup --update-token` to set it
   ```
   Ohhhh, since we haven't entered a token yet, pup asks us to do this.

8. Set the token: enter the token and press enter to confirm.
   ```shell
   $ pup --update-token
   Please input the new TOKEN:  
   ```
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

   If you have set TOKEN in pup, use `pup --delete-token` to delete it first.

   Then, execute the following script:

   ```shell
   sudo bash -c "$(curl -fsSL https://raw.githubusercontent.com/stevelauc/pup/main/script/uninstall.sh)"
   ```
   
   If you are in a place where you need proxy to access GitHub(i.e. China Mainland)
   , and you have `https_proxy` set in your personal configuration, use the following 
   script:

   ```shell
   sudo -E bash -c "$(curl -fsSL https://raw.githubusercontent.com/stevelauc/pup/main/script/uninstall.sh)"
   ```

# How it works
![workflow](https://github.com/SteveLauC/pic/blob/main/Page%201.jpeg)
