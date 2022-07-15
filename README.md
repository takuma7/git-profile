# git-profile

Managing [`user.name`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-username) and [`user.email`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-useremail) can be cumbersome if you have multiple profiles.
`git-profile` lets you create, manage, and switch between them with ease.

Optionally, you can also set [`user.signingkey`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-usersigningKey) for [signed commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

The quickest way to get started is to run `git profile import` and `git profile apply <your-profile>`:

```sh
$ git profile import
Found a user config as follows:
user.name=Git Smith
user.email=smith@work.com
✔ Enter profile name · work
✨ Successfully imported a git user as work

$ git profile apply work
✨ Successfully applied work
```

## Installation

### Cargo

If you haven't, install [`cargo`](https://github.com/rust-lang/cargo) as follows:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Then, run the following command to install `git-profile`:

```sh
cargo install gitprofile
```

(The actual executable will be named `git-profile`.)


## Usage

```
USAGE:
    git-profile [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config-path <CONFIG_PATH>    Use the given path to the configuration file to read/write
                                       profiles
    -h, --help                         Print help information
    -V, --version                      Print version information

SUBCOMMANDS:
    apply          Apply the given profile
    config-dump    Dump the content of the config file
    config-path    Print path to the config file
    current        Show the key or value of the current profile
    edit           Edit an existing profile
    help           Print this message or the help of the given subcommand(s)
    import         Import the current git config values as a profile
    list           List all profiles
    new            Create a new profile
    remove         Remove a profile
    rename         Rename the given profile with the given new name
    show           Show the details of the given profile
```

### Create a new profile

For a quick start, `git profile import` allows you to import your existing profile:

```sh
$ git profile import
Found a user config as follows:
user.name=Git Smith
user.email=smith@work.com
✔ Enter profile name · work
✨ Successfully imported a git user as work
```

Alternatively, you can manually create a new profile with the `git profile new` command:

```sh
$ git profile new
✔ Enter profile name · github
✔ Enter user name (user.name) · Git Smith
✔ Enter email (user.email) · smith@github.com
✔ Do you want to set signing key (user.signingkey) · no
✨ Created a new profile github
```

### Use a profile

To switch between profiles, use the `apply` subcommand:

```sh
$ git profile apply github
✨ Successfully applied github
```

Under the hood, this is equivalent to running the following command:

```sh
$ git config user.name "Git Smith"
$ git config user.email smith@github.com
```

`git-profile` supports the config level flags (`--local`, `--global`, `--system`, `--worktree`, and `--file`) out of the box.


### List existing profiles

`list` gives you an overview of your profiles. `*` indicates which profile is currently being activated.

```sh
$ git profile list
* work
  github
```

### Get the current profile key

`current` gives you the key of the current profile:

```sh
$ git profile current
work
```

### Show attributes

Use `show` to see the details of a profile:

```sh
$ git profile show github
user.name=Git Smith
user.email=smith@github.com
```
