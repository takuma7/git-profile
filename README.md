# git-profile

`git-profile` is a CLI tool that offers a handy way to manage git user profiles.

With `git-profile`, you can bundle [`user.name`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-username), [`user.email`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-useremail), and, optionally, [`user.signingkey`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-usersigningKey) into a named "profile."
You can create as many profiles as you wish and easily switch between them by key.


## Quick Overview

You can import your existing user config and name it as a new profile:

```sh
$ git profile import
Found a user config as follows:
user.name=Git Smith
user.email=smith@work.com
✔ Enter profile name · work
✨ Successfully imported a git user as work
```

Alternatively, you can also create new profiles manually as follows:

```sh
$ git profile new
✔ Enter profile name · github
✔ Enter user name (user.name) · Git Smith
✔ Enter email (user.email) · smith@github.com
✔ Do you want to set signing key (user.signingkey) · no
✨ Created a new profile github
```

The `list` subcommand gives you an overview of your profiles. `*` indicates which profile is currently being activated.

```sh
$ git profile list
* work
  github
```

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

## Installation

```sh
cargo install git-profile
```


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