use clap::Parser;
use config::Host;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use expectrl::Regex;

use std::process;

mod config;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {}

fn main() {
    let _ = Args::parse();

    let files = vec![".sshw", ".sshw.yml", ".sshw.yaml"];
    let home = dirs::home_dir().unwrap();
    let files = files
        .iter()
        .map(|&v| {
            let mut f = home.clone();
            f.push(v);
            f.to_str().unwrap().to_string()
        })
        .collect::<Vec<String>>();
    let files = files.iter().map(|v| v.as_str()).collect();

    let hosts = config::load(files).unwrap();

    let id = FuzzySelect::with_theme(&ColorfulTheme::default())
        // .with_prompt("select host:")
        .default(0)
        .items(&hosts)
        .interact();

    if id.is_err() {
        process::exit(1);
    }

    let host = hosts[id.unwrap()].clone();

    login(&host);
}

fn login(host: &Host) {
    let shell = host.to_ssh();
    let mut sh = expectrl::spawn(shell.clone()).expect("Error while spawning sh");

    println!("Connecting to {}", shell);

    if let Some(password) = host.password.clone() {
        if let Ok(c) = sh.expect(Regex("(?i)Password:")) {
            if !c.is_empty() {
                sh.send_line(&password).unwrap();
            }
        }
    }

    sh.interact().unwrap();
}
