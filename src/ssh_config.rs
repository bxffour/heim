use std::{fs, io};

pub fn parse_ssh_config(cfg_path: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(cfg_path)?;
    let res = parse_config(&content);
    Ok(res)
}

fn parse_config(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.starts_with("Host"))
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .parse::<String>()
                .unwrap()
        })
        .collect()
}
