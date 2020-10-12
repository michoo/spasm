extern crate log;

use clap::{App, Arg};

use colored::*;


mod hibp;
mod error;
mod vars;
mod utils;
mod command;
mod crypto;

use crate::error::SpasmError;
use std::fs;
use clap_generate::{generate, Generator};
use std::io;
use clap_generate::generators::{Bash,Elvish, Fish, PowerShell, Zsh};
use crate::crypto::backend::Entry;


#[tokio::main]
async fn main() -> Result<(), SpasmError> {
    env_logger::init();

    // Load all entries used in cli
    let path = utils::get_entries_path();

    let mut entries: Vec<Entry> = if path.is_file() {
        let packed = fs::read_to_string(&path)
            .expect(&format!("Unable to read from {}", path.display()));
        serde_json::from_str(&packed)
            .expect(&format!("Malformed {}", path.display()))
    } else {
        Vec::new()
    };


    // Get all arguments from cli
    let matches =build_cli().get_matches();


    // start to analyse args to run the right command
    // if in hell
    // else if in hell
    // else if in hell
    // else if in hell
    // else if in hell
    // else if in hell
    // sorry for than but it's a cli app...
    if let Some(breaches_email) = matches.value_of("breaches") {
        println!("{}","Validate if your email was leaked on Have I Been Pwnd".cyan());

        let breaches = hibp::breached_account::breached_account_full(breaches_email).await?;
        for breach in &breaches {
            println!(
                "Name: {:?} | Title: {:?} | Date: {:?}",
                breach.name, breach.title, breach.breach_date
            );
        }
    } else if matches.is_present("generate") {
        println!("{}","Random generated password ".cyan());
        let password = command::generate_password().await?;
        println!("{}", password.red());
        utils::is_good_password(password.clone()).await?;
        utils::to_clipboard(password.clone().into_bytes());

    } else if matches.is_present("dice") {
        println!("{}","Generated dice password".cyan());
        let password = command::generate_dice().await?;
        println!("{}", password.red());
        utils::is_good_password(password.clone()).await?;
        utils::to_clipboard(password.clone().into_bytes());

    } else if let Some(password) = matches.value_of("password") {
        println!("{}","Validate if your password is good".cyan());
        utils::is_good_password(password.to_string()).await?;

    }else if matches.is_present("add") {
        println!("{}","Add an entry".cyan());
        command::add(&mut entries).unwrap();
        entries.sort_unstable_by(|x, y| x.long.cmp(&y.long));
        let json = serde_json::to_string(&entries).unwrap();
        fs::write(&path, json)
            .expect(&format!("Unable to write to {}", path.display()));

    }else if matches.is_present("del") {
        println!("{}","Delete an entry".cyan());
        entries = command::del(entries).unwrap();
        entries.sort_unstable_by(|x, y| x.long.cmp(&y.long));

        let json = serde_json::to_string(&entries).unwrap();
        fs::write(&path, json)
            .expect(&format!("Unable to write to {}", path.display()));

    }else if matches.is_present("list") {
        println!("{}","List all entries".cyan());
        command::list(&entries);
    }else if matches.is_present("get") {
        println!("{}","Get a specific entry".cyan());
        command::get(&mut entries);
    }else if let Some(auto_completion) = matches.value_of("auto-completion") {
        let mut app = build_cli();
        eprintln!("Generating completion file for {}...", auto_completion);
        match auto_completion {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => panic!("Unknown generator"),
        }
    }
    Ok(())
}

fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, "spasm", &mut io::stdout());
}

fn build_cli() -> App<'static> {
    let banner: String = "
  ____  _____      __      ____    ___ ___
 /',__\\/\\ '__`\\  /'__`\\   /',__\\ /' __` __`\\
/\\__, `\\ \\ \\L\\ \\/\\ \\L\\.\\_/\\__, `\\/\\ \\/\\ \\/\\ \\
\\/\\____/\\ \\ ,__/\\ \\__/.\\_\\/\\____/\\ \\_\\ \\_\\ \\_\\
 \\/___/  \\ \\ \\/  \\/__/\\/_/\\/___/  \\/_/\\/_/\\/_/
          \\ \\_\\
           \\/_/ ".to_string().cyan().to_string();


    App::new(&banner)
        .about("Manage your identities")
        .version("0.1.0")

        // All application settings go here...
        // A simple "Flag" argument example (i.e. "-d") using the builder pattern
        .arg(
        Arg::new("breaches")
            .about("Validate if your email was leaked on Have I Been Pwnd")
            .short('b')
            .long("breaches")
            .takes_value(true),
        ).arg(
        Arg::new("password")
            .about("Validate if your password is good")
            .short('P')
            .long("password")
            .takes_value(true),
        ).arg(Arg::new("auto-completion").long("auto-completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ])).subcommand(
            App::new("generate")
                .about("Generate a password"),
        ).subcommand(
            App::new("dice")
                .about("Generate dice passphrase"),
        ).subcommand(
            App::new("add")
                .about("Add an entry"),
        ).subcommand(
            App::new("del")
                .about("Delete an entry"),
        ).subcommand(
            App::new("list")
                .about("List all entries"),
        ).subcommand(
            App::new("get")
                .about("Get an entry"),
        )
}
