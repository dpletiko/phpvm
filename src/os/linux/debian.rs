use anyhow::Error;
use std::process::Command;

pub fn use_version(version: String) -> Result<(), Error> {
    println!("You on Linux Debian, bro");

    println!("/usr/bin/php{}", version);

    let php = Command::new("update-alternatives")
        .args(&["--set", "php"])
        .arg(format!("/usr/bin/php{}", version))
        .output();

    match php {
        Ok(o) => println!("{:?}", String::from_utf8_lossy(&o.stdout)),
        Err(err) => return Err(Error::from(err))
    }

    let phar = Command::new("update-alternatives")
        .args(&["--set", "phar"])
        .arg(format!("/usr/bin/phah{}", version))
        .output();

    match phar {
        Ok(o) => println!("{:?}", String::from_utf8_lossy(&o.stdout)),
        Err(err) => return Err(Error::from(err))
    }

    let phar_phar = Command::new("update-alternatives")
        .args(&["--set", "phar.phar"])
        .arg(format!("/usr/bin/phar.phar{}", version))
        .output();

    match phar_phar {
        Ok(o) => println!("{:?}", String::from_utf8_lossy(&o.stdout)),
        Err(err) => return Err(Error::from(err))
    }

    let phpize = Command::new("update-alternatives")
        .args(&["--set", "phpize"])
        .arg(format!("/usr/bin/phpize{}", version))
        .output();

    match phpize {
        Ok(o) => println!("{:?}", String::from_utf8_lossy(&o.stdout)),
        Err(err) => return Err(Error::from(err))
    }

    let php_config = Command::new("update-alternatives")
        .args(&["--set", "php-config"])
        .arg(format!("/usr/bin/php-config{}", version))
        .output();

    match php_config {
        Ok(o) => println!("{:?}", String::from_utf8_lossy(&o.stdout)),
        Err(err) => return Err(Error::from(err))
    }

    Ok(())
}
