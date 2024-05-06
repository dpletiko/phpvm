use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::{fs::File, io::BufReader, process::{Command, Stdio}};

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
}

fn get_versions() -> Result<Vec<String>> {
    // TODO: Handle errors > match and stuff > Ok > Err
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
            .arg(r#"/php[0-9]/ { sub("php", "", $9); print $9 }"#)
            .stdin(ls)
            .output()
            .expect("Failed to find versions")
    };

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .lines()
        .map(String::from)
        .collect())
}

fn list_versions() -> Result<(), Error> {
    println!("Listing installed versions...");

    for version in get_versions()? {
       println!("{}", version);
    }

    Ok(())
}

fn use_composer() -> Result<(), Error> {
    println!("Using composer...");

    // let output = Command::new("jq")
    //     .arg(".require.php")
    //     .arg("composer.json")
    //     .output()
    //     .expect("Testing this shit?");
    //
    // if !output.stderr.is_empty() {
    //     eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    //     // TODO: error out!
    //     return;
    // }
    //
    // let version_range = String::from_utf8_lossy(&output.stdout);
    // println!("{}", version_range);

    let file = match File::open("composer.json") {
        Ok(file) => file,
        Err(err) => return Err(Error::msg(format!("Problem opening the file: {:?}", err)))
    };

    let reader = BufReader::new(file);
    let composer: Value = match serde_json::from_reader(reader) {
        Ok(it) => it,
        Err(err) => return Err(Error::msg(format!("Unable to parse file: {:?}", err)))
    };

    let version_constraint = match &composer["require"]["php"] {
        Value::String(v) if !v.is_empty() => v,
        _ => return Err(Error::msg("PHP Version not found!"))
    };

    println!("Found version constraint: {:?}", version_constraint);

    let ranges: Vec<&str> = version_constraint
        .trim()
        .trim_matches(|c| c == '\"')
        .split(" || ")
        .collect();

    let mut selected_version: Option<String> = None;
    for constraint in ranges {
        for version in get_versions()? {
            // Check if the version matches the constraint
            if version.starts_with(constraint.trim_start_matches('^')) {
                // Prioritize this version if it matches the constraint
                selected_version = Some(version);
            }
        }
    }

    match selected_version {
        Some(v) if !v.is_empty() => {
            println!("Version matched: {}", v);
            use_version(v)
        },
        _ => Err(Error::msg("Version not matched!"))
    }
}

fn use_version(version: String) -> Result<(), Error> {
    println!("Using version: {}", version);

    Ok(())
}
