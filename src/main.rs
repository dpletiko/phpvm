use anyhow::{Result, Error, bail};
use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
#[command(version, about)]
enum Commands {
    /// List installed versions
    #[clap(visible_alias = "ls")]
    List,

    // #[command(arg_required_else_help = true, after_help = "KAZZO", before_help = "BEFORE")]
    /// Set version as default
    Use {
        /// The version to use
        #[clap(required_unless_present("composer"))]
        version: Option<String>,

        /// Parse from composer if possible
        #[clap(long, short)]
        composer: bool,
    },
}

// fn validate_version_option(version: &Option<String>, composer: bool) -> Result<(), Error> {
//     if version.is_none() && !composer {
//         return Err(Error::raw(
//             clap::error::ErrorKind::MissingRequiredArgument,
//             "Version is required if --composer|-c is not specified")
//         )
//     }
//
//     Ok(())
// }
//
// impl Cli {
//     fn validate(&self) -> Result<(), Error> {
//         match &self.cmd {
//             Commands::Use { version, composer } => validate_version_option(version, *composer),
//             _ => Ok(())
//         }
//     }
// }

fn main() -> Result<()> {
    let cli = Cli::parse();

    // cli.validate()?;

    match cli.cmd {
        Commands::List => list_versions(),
        Commands::Use { version, composer } => match composer {
            true => use_composer(),
            false => use_version(version.unwrap()),
        },
    }

    Ok(())
}

fn list_versions() {
    println!("Listing installed versions...");

    let output = {
        let ls = Command::new("ls")
            .arg("-l")
            .arg("/usr/bin")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to list binaries")
            .stdout
            .expect("Failed to capture ls output");

        Command::new("awk")
            .arg(r#"/php/ { sub("php", "", $9); print $9 }"#)
            .stdin(ls)
            .output()
            .expect("Failed to find versions")
    };

    println!(
        "Installed versions: {}",
        String::from_utf8_lossy(&output.stdout)
    )
}

fn use_composer() {
    println!("Using composer...");

    let output = Command::new("jq")
        .arg(".require.php")
        .arg("composer.json")
        .output()
        .expect("Testing this shit?");

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    let version = String::from_utf8_lossy(&output.stdout);

    println!("{}", version);
}

fn use_version(version: String) {
    println!("{}", version)
}
