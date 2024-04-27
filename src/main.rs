use std::fs::File;
use std::io::{self, BufReader, Write};
use std::num::ParseIntError;
use std::process::Command;
use termimad::crossterm::style::Attribute::Underlined;
use termimad::crossterm::style::Color::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config_path: String,
}

use serde::Deserialize;
use termimad::MadSkin;

#[derive(Debug, Deserialize)]
struct Config {
    pub label: String,
    pub cmd: String,
}

#[derive(Debug, Deserialize)]
struct Group {
    pub group: String,
    config: Vec<Config>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Either {
    Config(Config),
    Group(Group),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Repr {
    Either(Vec<Either>),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Json parse failed: {0} \n")]
    JsonParse(#[from] serde_json::Error),

    #[error("Failed to open a file: {0} \n")]
    FileError(#[from] std::io::Error),

    #[error("{0}")]
    ParseError(#[from] ParseIntError),
}

fn exec() -> Result<(), Error> {
    let args = Args::parse();
    let f = File::open(args.config_path)?;
    let reader = BufReader::new(f);
    let json: Vec<Either> = serde_json::from_reader(reader)?;
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Red);

    println!("{}", skin.inline("**Cmd switch: **"));
    println!("{}", skin.inline("**===========**"));
    skin.bold.set_fg(White);
    let mut point = 0;
    for cfg in json.iter() {
        match cfg {
            Either::Config(conf) => {
                point += 1;
                skin.print_text(format!("**{})** {}", point, conf.label).as_str())
            }
            Either::Group(grp) => {
                skin.bold.add_attr(Underlined);
                skin.print_text(format!("\n**{}:**", &grp.group).as_str());
                skin.bold.remove_attr(Underlined);
                for config in grp.config.iter() {
                    point += 1;
                    skin.print_text(format!("  **{})** {}", point, config.label).as_str())
                }
                println!("");
            }
        }
    }
    skin.bold.set_fg(Red);
    println!("{}", skin.inline("**===========**"));

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let index = buffer.trim().parse::<usize>()?;

    let mut cmd = Command::new("sh");

    let flatten_cmd = json.iter().fold(vec![], |mut acc, cur| match cur {
        Either::Config(cfg) => {
            acc.push(cfg);
            acc
        }
        Either::Group(grp) => {
            for c in grp.config.iter() {
                acc.push(c);
            }
            acc
        }
    });

    match flatten_cmd.get(index - 1) {
        Some(conf) => {
            cmd.arg("-c").arg(conf.cmd.as_str());
        }
        None => {
            io::stderr().write_all("No such index \n".as_bytes())?;
        }
    }

    let output = cmd.output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

fn main() {
    match exec() {
        Ok(_) => {}
        Err(err) => {
            io::stderr().write_all(err.to_string().as_bytes()).unwrap();
        }
    }
}
