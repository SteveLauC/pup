### pup
A command line tool aiming to upload the local image used in your markdown file to
the GitHub repo and replace the local file path with the returned URL.

### How to use
1. Go to [token-settings](https://github.com/settings/tokens) to generate a new
token, make sure it has access to your picture repo.
2. Then clone this repo and build from source

```shell
$ git clone https://github.com/SteveLauC/pup.git
$ cd pup
$ cargo build
```
3. Move the compiled binary to the directory in your $PATH
```shell
$ cp target/debug/pup your_dir
```
4. Double check you have pup installed
```shell
$ which pup
some_path_to_pup
```
5. init config file
```shell
$ pup
name is unset.
repo is unset.
mail is unset.
token is unset.
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

[authorization]
token = "your-secret-token"
```
8. Time to enjoy
```shell
$ pup your-markdown-file.md
```


### How it works
![workflow](https://github.com/SteveLauC/pic/blob/main/Page%201.png)
