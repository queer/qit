use std::{error::Error, process::Command};

use clap::{App, Arg};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let app = App::new("qit")
        .subcommand(
            App::new("commit")
                .alias("c")
                .about("Commits with a meaningful commit message")
                .arg(
                    Arg::new("type")
                        .help("The type of commit")
                        .possible_values(vec![
                            "chore", "feature", "refactor", "fix", "test", "style", "doc",
                        ])
                        .required(true),
                )
                .arg(
                    Arg::new("area")
                        .help("The section of the code this commit focuses on")
                        .long("area")
                        .short('a')
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::new("message")
                        .help("The commit message")
                        .required(true),
                ),
        )
        .subcommand(App::new("status").about("Checks the current status of the repo"));

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("commit", args)) => {
            let type_ = args.value_of("type").unwrap();
            let area = args.value_of("area");
            let message = args.value_of("message").unwrap();
            let formatted = match area {
                Some(area) => format!("{}({}): {}", type_, area, message),
                None => format!("{}: {}", type_, message),
            };
            println!("Committing: {}", formatted);
            commit(&formatted)?;
        }
        _ => panic!("aaaaaaa"),
    }
    Ok(())
}

fn commit(message: &String) -> Result<()> {
    Command::new("git")
        .arg("add")
        .arg("-A")
        .arg("*")
        .arg(".*")
        .spawn()?
        .wait()?;
    Command::new("git")
        .arg("commit")
        .arg("-am")
        .arg(message)
        .spawn()?
        .wait()?;
    Ok(())
}