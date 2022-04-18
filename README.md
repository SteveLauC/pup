### Pup [![BUILD](https://github.com/stevelauc/pup/workflows/Rust/badge.svg)](https://github.com/stevelauc/pup/actions/workflows/build.yml)
A command line tool aiming to upload the local image used in your markdown file to
the GitHub repo and replace the local file path with the returned URL.

### TO DO
- [x] Use system password management to store TOKEN
- [ ] Support for Windows 
- [x] Multithreading file manipulation
- [ ] Relative image path support

### Demo video
![demo](https://user-images.githubusercontent.com/96880612/163778336-a2fda462-0af0-45fa-afb5-bbec48b438fa.gif)

### How to use
1. Go to [token-settings](https://github.com/settings/tokens) to generate a new
token, make sure it has access to your picture repo.
2. Then clone this repo and build from source
> There is a prebuilt version of pup under [prebuilt-binary](https://github.com/SteveLauC/pup/tree/main/prebuilt-binary), you can download it directly instead of building from source.

```shell
$ git clone https://github.com/SteveLauC/pup.git
$ cd pup
$ cargo build
```
3. Move the compiled binary to the directory in your $PATH, for example:
```shell
$ cp target/debug/pup /usr/share/bin
```
4. Double check you have pup installed
```shell
$ which pup
/usr/local/bin/pup
```
5. init config file
```shell
$ pup
name is unset.
repo is unset.
mail is unset.
```
6. On your first run, `pup` will create a configuration file under
`$HOME/.config/pup` and complain about the incompleteness of the configuration
to you.
7. Then edit it to make it complete.
```shell
$ vim $HOME/.config/pup/config.toml
```
Make it something like the following one, remeber to replace each field with
your own one.
> Currently the token is stored in plain text, will fix it later!
```
# configuration file for pup
[user]
github-user-name = "SteveLauC"
github-repo-name = "test-repo"
mail = "stevelauc@outlook.com"
```
8. Let's try it again!
```shell
$ pup
Please enter the TOKEN: 
```
Ohhhh, since we haven't entered a token yet, pup asks us to do this.

9. All configuration is done! Time to enjoy:)
```shell
$ pup your-markdown-file.md
```


### How it works
![workflow](https://github.com/SteveLauC/pic/blob/main/Page%201.png)
