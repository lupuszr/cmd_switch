use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::Command;
use termimad::crossterm::style::Attribute::Underlined;
use termimad::crossterm::style::Color::*;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
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

fn main() {
    let args = Args::parse();
    let file = File::open(args.config_path);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let json: Vec<Either> = serde_json::from_reader(reader).unwrap();
            let mut skin = MadSkin::default();
            skin.bold.set_fg(Red);

            println!("{}", skin.inline("**Invoke: **"));
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
            stdin.read_line(&mut buffer).unwrap();

            println!("{buffer}");

            let index = buffer.trim().parse::<usize>().unwrap();

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
                    io::stderr()
                        .write_all("No such index \n".as_bytes())
                        .unwrap();
                }
            }

            let output = cmd.output().unwrap();
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(_) => todo!(),
    }
    // let test = r#"[
    //     {
    //         "label": "hello",
    //         "cmd": "cargo run"
    //     },
    //     {
    //         "label": "echo Hi",
    //         "cmd": "ls -la"
    //     }]"#;
    // let json: Vec<Config> = serde_json::from_str(test).unwrap();

    // let error = echo_hello.stderr();
    // let s = String::from_utf8(output).unwrap();
    // println!("{}", s);
}
