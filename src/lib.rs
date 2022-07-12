use std::path::PathBuf;

use anyhow;
use clap::{ArgGroup, Parser, Subcommand};
use commands::current::ProfileProperty;
use config::{AppConfigClient, DEFAULT_FILE_NAME};
use context::AppContext;
use directories::ProjectDirs;
use git::{GitConfigClient, Level};

pub mod commands;
pub mod config;
pub mod context;
pub mod git;
pub mod validation;

#[derive(Parser, Debug)]
#[clap(name = "git-profile")]
#[clap(author, version, about = "A tool to easily configure git user profiles", long_about = None)]

pub struct CliArgs {
    /// Use the given path to the configuration file to read/write profiles
    #[clap(short = 'c', long = "config-path")]
    pub config_path: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show the key or value of the current profile
    #[clap(group(
        ArgGroup::new("current")
            .args(&["name", "email", "signingkey", "profile"]))
    )]
    #[clap(group(
        ArgGroup::new("current_level")
            .args(&["system", "global", "local", "worktree", "file"]))
    )]
    Current {
        /// Show user.name of the current profile
        #[clap(short, long, action)]
        name: bool,
        /// Show user.email of the current profile
        #[clap(short, long, action)]
        email: bool,
        /// Show user.signingkey of the current profile if any
        #[clap(short, long, action)]
        signingkey: bool,
        /// Show the current profile
        #[clap(short = 'P', long, action)]
        profile: bool,
        /// Show the name of the current profile (default)
        #[clap(short, long, action)]
        profile_key: bool,

        /// Read only from system-wide `$(prefix)/etc/gitconfig` rather than from all available files.
        #[clap(long, action)]
        system: bool,

        /// Read only from `~/.gitconfig` and from `$XDG_CONFIG_HOME/git/config` rather than from all available files.
        #[clap(long, action)]
        global: bool,

        /// Read only from the repository `.git/config` rather than from all available files.
        #[clap(long, action)]
        local: bool,

        /// Similar to `--local` except that `$GIT_DIR/config.worktree` is read from if `extensions.worktreeConfig` is enabled. If not it's the same as `--local`.
        #[clap(long, action)]
        worktree: bool,

        /// Read only from the specified file rather than from all available files.
        #[clap(long, action)]
        file: Option<String>,
    },
    /// List all profiles
    #[clap(group(
        ArgGroup::new("current_level")
            .args(&["system", "global", "local", "worktree", "file"]))
    )]
    List {
        /// Read only from system-wide `$(prefix)/etc/gitconfig` rather than from all available files.
        #[clap(long, action)]
        system: bool,

        /// Read only from `~/.gitconfig` and from `$XDG_CONFIG_HOME/git/config` rather than from all available files.
        #[clap(long, action)]
        global: bool,

        /// Read only from the repository `.git/config` rather than from all available files.
        #[clap(long, action)]
        local: bool,

        /// Similar to `--local` except that `$GIT_DIR/config.worktree` is read from if `extensions.worktreeConfig` is enabled. If not it's the same as `--local`.
        #[clap(long, action)]
        worktree: bool,

        /// Read only from the specified file rather than from all available files.
        #[clap(long, action)]
        file: Option<String>,
    },
    /// Show the details of the given profile
    Show {
        #[clap(value_parser)]
        profile_key: String,
    },
    /// Create a new profile
    New,
    /// Edit an existing profile
    Edit {
        #[clap(value_parser)]
        profile_key: Option<String>,
    },
    /// Remove a profile
    Remove {
        #[clap(value_parser)]
        profile_key: String,
    },
    /// Rename the given profile with the given new name
    Rename {
        /// Old profile key
        #[clap(value_parser)]
        old: String,
        /// New profile key
        #[clap(value_parser)]
        new: String,
    },
    /// Apply the given profile
    #[clap(group(
        ArgGroup::new("apply_level")
            .args(&["global", "system", "local", "worktree", "file"]))
    )]
    Apply {
        #[clap(value_parser)]
        profile_key: String,

        /// Read only from system-wide `$(prefix)/etc/gitconfig` rather than from all available files.
        #[clap(long, action)]
        system: bool,

        /// Read only from `~/.gitconfig` and from `$XDG_CONFIG_HOME/git/config` rather than from all available files.
        #[clap(long, action)]
        global: bool,

        /// Read only from the repository `.git/config` rather than from all available files.
        #[clap(long, action)]
        local: bool,

        /// Similar to `--local` except that `$GIT_DIR/config.worktree` is read from if `extensions.worktreeConfig` is enabled. If not it's the same as `--local`.
        #[clap(long, action)]
        worktree: bool,

        /// Read only from the specified file rather than from all available files.
        #[clap(long, action)]
        file: Option<String>,
    },
    /// Import the current git config values as a profile
    #[clap(group(
        ArgGroup::new("import_level")
            .args(&["global", "system", "local", "worktree", "file"]))
    )]
    Import {
        /// Read only from system-wide `$(prefix)/etc/gitconfig` rather than from all available files.
        #[clap(long, action)]
        system: bool,

        /// Read only from `~/.gitconfig` and from `$XDG_CONFIG_HOME/git/config` rather than from all available files.
        #[clap(long, action)]
        global: bool,

        /// Read only from the repository `.git/config` rather than from all available files.
        #[clap(long, action)]
        local: bool,

        /// Similar to `--local` except that `$GIT_DIR/config.worktree` is read from if `extensions.worktreeConfig` is enabled. If not it's the same as `--local`.
        #[clap(long, action)]
        worktree: bool,

        /// Read only from the specified file rather than from all available files.
        #[clap(long, action)]
        file: Option<String>,
    },
    /// Dump the content of the config file
    ConfigDump,
    /// Print path to the config file
    ConfigPath,
}

fn get_default_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("org", "git-profile", "git-profile").unwrap();
    PathBuf::from(project_dirs.config_dir().join(DEFAULT_FILE_NAME))
}

fn get_config_path(args: &CliArgs) -> PathBuf {
    if let Some(config_path) = &args.config_path {
        PathBuf::from(config_path)
    } else {
        get_default_path()
    }
}

fn get_level(
    system: bool,
    global: bool,
    local: bool,
    worktree: bool,
    file: Option<String>,
) -> Option<Level> {
    match (system, global, local, worktree, file) {
        (true, _, _, _, _) => Some(Level::System),
        (_, true, _, _, _) => Some(Level::Global),
        (_, _, true, _, _) => Some(Level::Local),
        (_, _, _, true, _) => Some(Level::WorkTree),
        (_, _, _, _, Some(file_path)) => Some(Level::File(file_path)),
        _ => None,
    }
}

pub fn run() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let config_path = get_config_path(&args);

    let context = AppContext {
        git_config_client: Box::new(GitConfigClient::new()),
        config_client: Box::new(AppConfigClient::new(config_path.clone())),
    };

    match args.command {
        Commands::Current {
            // What to show
            name,
            email,
            signingkey,
            profile,
            profile_key,
            // Which level to retrieve from
            system,
            global,
            local,
            worktree,
            file,
        } => {
            let selected_property = match (name, email, signingkey, profile, profile_key) {
                (true, _, _, _, _) => ProfileProperty::Name,
                (_, true, _, _, _) => ProfileProperty::Email,
                (_, _, true, _, _) => ProfileProperty::SigningKey,
                (_, _, _, true, _) => ProfileProperty::Profile,
                (_, _, _, _, true) | _ => ProfileProperty::ProfileKey,
            };
            commands::current::execute(
                &context,
                &selected_property,
                &get_level(system, global, local, worktree, file)
            )?;
        }
        Commands::List {
            // Which level to retrieve from
            system,
            global,
            local,
            worktree,
            file,
        } => {
            commands::list::execute(
                &context,
                &get_level(system, global, local, worktree, file)
            )?;
        }
        Commands::Show { profile_key } => {
            commands::show::execute(&context, &profile_key)?;
        }
        Commands::New {} => {
            commands::new::execute(&context)?;
        }
        Commands::Edit { profile_key } => {
            commands::edit::execute(&context, &profile_key)?;
        }
        Commands::Remove { profile_key } => {
            commands::remove::execute(&context, &profile_key)?;
        }
        Commands::Rename { old, new } => {
            let old = old.trim();
            let new = new.trim();
            commands::rename::execute(&context, old, new)?;
        }
        Commands::Apply {
            profile_key,
            // Levels
            system,
            global,
            local,
            worktree,
            file,
        } => {
            commands::apply::execute(
                &context,
                &profile_key,
                &get_level(system, global, local, worktree, file)
            )?;
        }
        Commands::Import {
            system,
            global,
            local,
            worktree,
            file,
        } => {
            commands::import::execute(
                &context, 
                &get_level(system, global, local, worktree, file)
            )?;
        },
        Commands::ConfigDump {} => {
            commands::config_dump::execute(&config_path.to_str().unwrap())?;
        },
        Commands::ConfigPath {} => {
            println!("{}", &config_path.display());
        }
    }

    Ok(())
}
