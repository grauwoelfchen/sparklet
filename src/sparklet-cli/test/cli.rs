#[cfg(test)]
mod test {
    use std::env;
    use std::error::Error;

    use assert_cmd::Command;
    use predicates::prelude::*;

    const BIN_NAME: &str = "sparklet-cli";

    #[test]
    fn test_version() -> Result<(), Box<dyn Error>> {
        let version = env::var("CARGO_PKG_VERSION")?;
        let expected = &format!("Sparklet CLI {}", version);

        for flag in &["-V", "--version"] {
            Command::cargo_bin(BIN_NAME)?
                .arg(flag)
                .assert()
                .success()
                .stdout(predicate::str::contains(expected));
        }
        Ok(())
    }
}
