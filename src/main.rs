use std::env;
use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::process::Command;

use clap::{App, Arg};
use git2::Repository;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let app = App::new("qit")
        // Commit
        .subcommand(
            App::new("commit")
                .alias("c")
                .about("Commits with a meaningful commit message")
                .after_help("
Format:
    <emoji> <type>[(<area>)]: <message>

Emojis:
       chore 🚧
     feature ✨
    refactor ♻️
         fix 🐛
        test ✅
       style 🎨
         doc 📝
        deps 📦
      deploy 🚀
    
    Emojis inspired by https://gitmoji.dev/

Examples:
    ✨ feature: Add thing
    ✨ feature(cli): Improve args
    🚧 chore: Do thing
    🚀 deploy(api): Deploy to production
                ")
                .arg(
                    Arg::new("type")
                        .help("The type of commit")
                        .possible_values(vec![
                            "chore", "feature", "refactor", "fix", "test", "style", "doc", "deps", "deploy",
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
        // Push
        .subcommand(
            App::new("push")
                .alias("p")
                .about("Pushes the current branch to the remote. Will not push if there are uncommitted changes.")
                .arg(
                    Arg::new("force")
                        .help("Force push. Ignores uncommitted changes. **WARNING**: This is the same as `git push -f`!")
                        .long("force")
                        .short('f')
                        .takes_value(false),
                ),
        )
        // Undo
        .subcommand(
            App::new("undo")
                .alias("u")
                .about("Undoes the last commit"),
        )
        // Log
        .subcommand(
            App::new("log").alias("l").about("Shows the git log").arg(
                Arg::new("short")
                    .long("short")
                    .short('s')
                    .help("Whether to show a shortened git log"),
            ),
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("commit", args)) => {
            let type_ = args.value_of("type").unwrap();
            let area = args.value_of("area");
            let message = args.value_of("message").unwrap();
            handle(commit(type_, &area, message));
        }
        Some(("log", args)) => {
            let short = args.is_present("short");
            handle(log(short));
        }
        Some(("push", args)) => {
            let force = args.is_present("force");
            handle(push(force));
        }
        Some(("undo", _)) => handle(undo()),
        _ => panic!("aaaaaaa"),
    }
    Ok(())
}

fn handle(res: Result<()>) {
    match res {
        Ok(_) => (),
        Err(err) => {
            eprintln!("💥 Unable to run command:");
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
}

// Subcommands //

fn commit(type_: &str, area: &Option<&str>, message: &str) -> Result<()> {
    // Emojis inspired by https://gitmoji.dev/
    let emoji = match type_ {
        "chore" => "🚧",
        "feature" => "✨",
        "refactor" => "♻️",
        "fix" => "🐛",
        "test" => "✅",
        "style" => "🎨",
        "doc" => "📝",
        "deps" => "📦",
        "deploy" => "🚀",
        _ => {
            panic!("Unknown commit type")
        }
    };
    let emoji = match env::var("QIT_DISABLE_EMOJIS") {
        Ok(value) => {
            if value == "true" {
                ""
            } else {
                emoji
            }
        }
        _ => emoji,
    };
    let formatted = match area {
        Some(area) => format!("{} {}({}): {}", emoji, type_, area, message),
        None => format!("{} {}: {}", emoji, type_, message),
    };
    let formatted = formatted.trim();

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
        .arg(formatted)
        .spawn()?
        .wait()?;
    Ok(())
}

fn log(short: bool) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("log");
    if short {
        cmd.arg("--oneline");
    }
    cmd.spawn()?.wait()?;
    Ok(())
}

fn push(force: bool) -> Result<()> {
    let pending_changes = if let Ok(count) = repo_status() {
        count > 0
    } else {
        false
    };

    if pending_changes && !force {
        return Err(IOError::new(ErrorKind::Other, "There are uncommitted changes").into());
    }
    let mut cmd = Command::new("git");
    cmd.arg("push");
    if force {
        cmd.arg("--force");
    }
    cmd.spawn()?.wait()?;
    Ok(())
}

fn undo() -> Result<()> {
    Command::new("git")
        .arg("reset")
        .arg("--soft")
        .arg("HEAD~1")
        .spawn()?
        .wait()?;
    Ok(())
}

// Helpers //

fn repo_status() -> Result<usize> {
    let repo = Repository::open(".")?;
    let modified_files = repo
        .statuses(Some(git2::StatusOptions::new().include_untracked(true)))?
        .iter()
        .filter(|s| !s.status().is_ignored())
        .count();
    Ok(modified_files)
}
