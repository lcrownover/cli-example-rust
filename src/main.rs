use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("Example CLI")
        .author("Lucas Crownover")
        .version("0.0.1")
        .about("Shows what an example CLI looks like in Clap for Rust (builder pattern)")
        .arg(
            Arg::new("file")
                .short('f')
                .value_parser(clap::value_parser!(PathBuf))
                .help("The file to read from")
                .required(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .action(ArgAction::SetTrue)
                .help("Print debug information verbosely"),
        )
        .subcommands([
            Command::new("sub1")
                .about("Subcommand 1: Makes Sounds")
                .arg(
                    Arg::new("sound")
                        .short('s')
                        .required(true)
                        .help("The sound to make"),
                )
                .arg(
                    Arg::new("volume")
                        .short('v')
                        .value_parser(clap::value_parser!(f64))
                        .required(false)
                        .help("The volume of the sound")
                        .default_value("1.0"),
                ),
            Command::new("sub2")
                .about("Subcommand 2: Says Greetings")
                .arg(
                    Arg::new("greeting")
                        .short('g')
                        .required(false)
                        .help("The greeting to say"),
                )
                .arg(
                    Arg::new("times")
                        .short('t')
                        .required(false)
                        .value_parser(clap::value_parser!(u32))
                        .help("How many times to say the greeting")
                        .default_value("1"),
                ),
        ])
        .get_matches();

    if let Some(file_path) = matches.get_one::<PathBuf>("file") {
        println!("Value for file path: {}", file_path.display());
    }

    if matches.get_flag("debug") {
        println!("Debug mode activated");
    }

    if let Some(ref matches) = matches.subcommand_matches("sub1") {
        if let Some(sound) = matches.get_one::<String>("sound") {
            println!("Value for sound: {}", sound);
        }

        if let Some(volume) = matches.get_one::<f64>("volume") {
            println!("Value for volume: {}", volume);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("sub2") {
        let default_greeting = "Hello, world!".to_string();
        let mut greeting = default_greeting.clone();
        if let Some(g) = matches.get_one::<String>("greeting") {
            println!("Value for greeting: {}", g);
            greeting = g.to_string();
        };

        let mut times = 0;
        if let Some(t) = matches.get_one::<u32>("times") {
            println!("Value for times: {}", t);
            times = *t;
        }

        for _ in 0..times {
            println!("{}", greeting);
        }
    }
}
