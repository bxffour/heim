use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io;

use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ssh_config path
    #[arg(short, long, default_value = "")]
    cfg: String,
}

struct Cli {
    ssh_config_path: String,
}

impl Cli {
    fn new(ssh_config_path: String) -> Self {
        Cli { ssh_config_path }
    }

    fn get_hosts(&self) -> Result<Vec<String>, io::Error> {
        ssh_config::parse_ssh_config(&self.ssh_config_path)
    }
}

fn main() {
    let args = Args::parse();

    let ssh_config = match args.cfg.as_str() {
        "" => dirs::home_dir()
            .expect("error retrieving home directory")
            .join(".ssh/config")
            .to_string_lossy()
            .into_owned(),
        _ => args.cfg.to_string(),
    };

    let cli = Cli::new(ssh_config);
    let hosts = cli.get_hosts().unwrap();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a host")
        .default(0)
        .items(&hosts[..])
        .interact()
        .expect("error getting fuzzy selector");

    let mut connection = Command::new("ssh");

    let mut process = connection
        .arg(hosts[selection].as_str())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to run ssh connection.");

    process.wait().expect("command panicked");
}
