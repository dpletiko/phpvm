use anyhow::Error;

pub fn use_version(version: String) -> Result<(), Error> {
    println!("You on Windows, bro");

    Err(Error::msg("Exitting!"))
}
