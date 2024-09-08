use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();

    let inner = || {
        let input = std::fs::read_to_string(&cli.file)
            .map_err(|io| Error::ReadFile(cli.file.clone(), io))?;

        let compose: docker_compose_types::ComposeFile = toml::from_str(&input)?;
        let yaml = serde_yml::to_string(&compose).expect("serializing should not fail");

        if cli.write {
            let name = cli.file.with_extension("yaml");
            std::fs::write(&name, yaml).map_err(Error::WriteFile)?;
            println!(
                "compose-toml: {} {}{}",
                "wrote to".green(),
                name.display(),
                "!".green()
            )
        } else {
            print!("{yaml}")
        }

        Ok::<(), Error>(())
    };

    if let Err(e) = inner() {
        if cli.debug {
            println!("compose-toml: {e:?}")
        } else {
            println!("compose-toml {} :(", e.to_string().red())
        }
    };
}

#[derive(clap::Parser)]
struct Cli {
    #[arg(default_value = "compose.toml")]
    file: PathBuf,

    #[arg(short, long)]
    write: bool,

    #[arg(short, long)]
    debug: bool,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("couldn't read input file `{0}`")]
    ReadFile(PathBuf, std::io::Error),

    #[error("failed to parse TOML")]
    ParseToml(#[from] toml::de::Error),

    #[error("couldn't write file to disk")]
    WriteFile(std::io::Error),
}
