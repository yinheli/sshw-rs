use clap::Parser;
use config::Host;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use expectrl::Regex;

use std::{process, time::Duration};

mod config;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(long)]
    verbose: bool,
}

fn main() {
    let cli = Args::parse();

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

    let opt = if cli.verbose { Some("-vvv") } else { None };

    login(&host, opt);
}

fn login(host: &Host, opt: Option<&str>) {
    let shell = host.to_ssh(opt);
    let mut sh = expectrl::spawn(shell.clone()).expect("Error while spawning sh");
    sh.set_expect_lazy(true);
    sh.set_echo(true, Some(Duration::from_millis(100))).unwrap();
    sh.set_expect_timeout(Some(Duration::from_secs(3)));

    let termsize::Size { rows, cols } = termsize::get().unwrap();
    sh.set_window_size(cols, rows).unwrap();

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
