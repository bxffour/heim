use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ssh_config path
    #[arg(short, long)]
    cfg: String,
}

struct Cli {
    ssh_config_path: String,
}

use ssh_config;
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
    let cli = Cli::new(args.cfg);
    let hosts = cli.get_hosts().unwrap();

    // let host_refs: Vec<&str> = hosts.iter().map(|s| s.as_str()).collect();
    let hr: Vec<&str> = hosts.iter().map(AsRef::as_ref).collect();
    let hr: &[&str] = hr.as_slice();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a host")
        .default(0)
        .item(hr)
        .interact()
        .unwrap();
}
