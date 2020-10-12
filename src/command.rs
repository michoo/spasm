
use crate::utils;
use rpassword::prompt_password_stdout;
use std::io;
use std::str;
use std::fs::File;
use std::collections::HashSet;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::OsRng;
use rand::{thread_rng, Rng};
use std::io::Read;
use crate::error::SpasmError;
use colored::*;
use crate::crypto::backend::{Entry, open, seal};


pub fn list(entries: &Vec<Entry>) {
    for entry in entries {
        println!("{}", entry);
    }
}

pub fn del(mut entries: Vec<Entry>) ->  io::Result<Vec<Entry>>{
    // get entry name
    let mut long;

    loop {
        long = utils::prompt_string("Entry name: ")?;
        if long.is_empty() {
            println!("Entry name can't be empty.");
            continue;
        }
        break;
    }

    let index = entries.iter().position(|entry| long == entry.long || long == entry.short).unwrap();
    entries.remove(index);

    // open and decrypt password
    Ok(entries)
}

pub fn get(entries: &mut Vec<Entry>)-> io::Result<()>{
    // get entry name
    let mut long;

    loop {
        long = utils::prompt_string("Entry name: ")?;
        if long.is_empty() {
            println!("Entry name can't be empty.");
            continue;
        } else if entries.iter()
            .any(|entry| long == entry.long || long == entry.short) {
            println!("Name exist");
        }
        break;
    }

    // find existing entry
    let entri = entries.iter()
        .find(|entry| long == entry.long || long == entry.short).unwrap();

    // get master password
    let mut master;
    loop {
        master = prompt_password_stdout("Master key: ")?;
        if master.is_empty() {
            println!("Master key can't be empty.");
            continue;
        }

        break;
    }

    match open(&entri, &master) {
        Ok(password) => {
            utils::to_clipboard(password);
        },
        Err(_) => {
            println!("Either master key was incorrect or entry is tampered.");

        }
    }
    // let password = backend::open(&entri,&master).unwrap();
    // let password_string = str::from_utf8(&password).unwrap();
    // println!("{}", password_string);

    // validate it's
    Ok(())
}

pub fn add(entries: &mut Vec<Entry>) -> io::Result<()> {

    let mut rng = thread_rng();
    let mut long;
    loop {
        long = utils::prompt_string("Entry name: ")?;
        if long.is_empty() {
            println!("Entry name can't be empty.");
            continue;
        } else if entries.iter()
            .any(|entry| long == entry.long || long == entry.short) {
            println!("Name already in use.");
            continue;
        }
        break;
    }

    let mut short;
    loop {
        short = utils::prompt_string("Shortened name (optional): ")?;
        if !short.is_empty() && entries.iter()
            .any(|entry| short == entry.long || short == entry.short) {
            println!("Name already in use.");
            continue;
        }
        break;
    }
    let extra = utils::prompt_string("login (optional): ")?;


    let mut password;
    let mut confirm;
    loop {
        password = prompt_password_stdout("Password: ")?;
        confirm = prompt_password_stdout("Enter again: ")?;
        if password != confirm {
            println!("Passwords don't match.");
            continue;
        }
        break;
    }

    let mut master;
    let mut confirm;
    loop {
        master = prompt_password_stdout("Master key: ")?;
        if master.is_empty() {
            println!("Master key can't be empty.");
            continue;
        }
        confirm = prompt_password_stdout("Enter again: ")?;
        if master != confirm {
            println!("Master keys don't match.");
            continue;
        }
        break;
    }

    let mut salt = [0u8; 16];
    rng.fill(&mut salt);
    let mut entry = Entry {
        long: long,
        short: short,
        extra: extra,
        salt: salt,
        sealed: password.as_bytes().to_vec(),
    };
    seal(&mut entry, &master).unwrap();
    entries.push(entry);

    println!("Success! Entry added.");
    utils::to_clipboard(password.clone().into_bytes());
    Ok(())
}


pub(crate) async fn generate_dice() -> Result<String, SpasmError>{
    // Get how many words
    let mut char_number: String;
    loop {
        char_number = utils::prompt_string("How many words?  ")?;
        if char_number.is_empty() {
            println!("Answer can't be empty.");
            continue;
        }
        break;
    }
    let dice_number: usize = char_number.parse().unwrap();

    // Load word list
    let full_path_string = utils::get_dice_path();
    let mut file = File::open(full_path_string.as_path())?;
    let mut wordlist = String::new();
    file.read_to_string(&mut wordlist)?;

    let dict: Vec<&str> = wordlist
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let mut sampler = Uniform::from(0..dict.len()).sample_iter(OsRng);
    let separator = " ";
    let mut new_password: String = "anfdvKjqoiq[i$#h6&7".to_string();
    for _ in 0..1 {
        let password = sampler
            .by_ref()
            .take(dice_number as usize)
            .map(|i| dict[i])
            .collect::<Vec<&str>>()
            .join(&separator);
        new_password = password.clone();
    }
    Ok(new_password)
}

pub(crate) async fn generate_password() -> Result<String, SpasmError>{
    let mut rng = thread_rng();
    static LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    static UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    static DIGIT: &str = "0123456789";
    static SYMBOL: &str = "#$%&*+,-./:;<=>?@[\\]^_{|}~";


    let mut charset = String::new();
    let mut password = String::new();

    let mut char_number: String;
    loop {
        char_number = utils::prompt_string("How many chars?  ")?;
        if char_number.is_empty() {
            println!("Answer can't be empty.");
            continue;
        }
        break;
    }
    let length: usize = char_number.parse().unwrap();

    let mut lower_dictionnary: String;
    loop {
        lower_dictionnary = utils::prompt_string("Do you want to use lower cases? (Y/n) ")?;
        if lower_dictionnary.is_empty() {
            lower_dictionnary = "Y".to_string();
        }
        break;
    }
    if lower_dictionnary.to_uppercase() == "Y"||
        lower_dictionnary.to_uppercase() == "YES"{
        charset.push_str(LOWER);
    }


    let mut upper_dictionnary: String;
    loop {
        upper_dictionnary = utils::prompt_string("Do you want to use upper cases? (Y/n) ")?;
        if upper_dictionnary.is_empty() {
            upper_dictionnary = "Y".to_string();
        }
        break;
    }
    if upper_dictionnary.to_uppercase() == "Y"||
        upper_dictionnary.to_uppercase() == "YES"{
        charset.push_str(UPPER);
    }


    let mut digit_dictionnary: String;
    loop {
        digit_dictionnary = utils::prompt_string("Do you want to use digit chars? (Y/n) ")?;
        if digit_dictionnary.is_empty() {
            digit_dictionnary = "Y".to_string();
        }
        break;
    }
    if digit_dictionnary.to_uppercase() == "Y"||
        digit_dictionnary.to_uppercase() == "YES"{
        charset.push_str(DIGIT);
    }

    let mut symbol_dictionnary: String;
    loop {
        symbol_dictionnary = utils::prompt_string("Do you want to use symbol chars? (Y/n) ")?;
        if symbol_dictionnary.is_empty() {
            symbol_dictionnary = "Y".to_string();
        }
        break;
    }
    if symbol_dictionnary.to_uppercase() == "Y"||
        symbol_dictionnary.to_uppercase() == "YES"{
        charset.push_str(SYMBOL);
    }

    println!("Dictionnary used: {}", charset.green());

    for _ in 0..length {
        let n = rng.gen_range(0, charset.len());
        let value = charset.chars().nth(n).unwrap().to_string();
        password.push_str(&*value);
    }

    println!("Password generated!");

    Ok(password.clone())
}