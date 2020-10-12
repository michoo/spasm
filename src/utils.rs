use regex::Regex;
use colored::*;
use crate::error::SpasmError;
use std::{io, env};
use std::io::prelude::*;

use clipboard_ext::x11_bin::ClipboardContext;
use clipboard_ext::clipboard::ClipboardProvider;
use crate::{hibp, vars};
use std::path::{Path, PathBuf};


pub(crate) fn entropy(shannon: &str)-> bool{
    // entropy H = log (count_alphabet**count)/log2
    //https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-63-2.pdf page 104
    let lenght_password= shannon.chars().count() as f64;
    let mut dictionnary_count = 0.0_f64;
    let re_lower = Regex::new(r"[a-z]").unwrap();
    let result_lower = re_lower.is_match(shannon);
    let lower_count = 26.0_f64;
    if result_lower{
        dictionnary_count+=lower_count;
    }
    let re_upper = Regex::new(r"[A-Z]").unwrap();
    let result_upper = re_upper.is_match(shannon);
    let upper_count = 26.0_f64;
    if result_upper{
        dictionnary_count+=upper_count;
    }
    let re_number = Regex::new(r"[0-9]").unwrap();
    let result_number = re_number.is_match(shannon);
    let number_count = 10.0_f64;
    if result_number{
        dictionnary_count+=number_count;
    }
    let re_punct = Regex::new(r"[#$%&*+,-./:;<=>?@[\\]^_{|}~]").unwrap(); //   !-/:-@\[-`{-~] 14 chars ;#?&$%*_!:@~
    let result_punct = re_punct.is_match(shannon);
    let punct_count = 26.0_f64;
    if result_punct{
        dictionnary_count+=punct_count;
    }

    let entropy:f64 = (dictionnary_count.powi(lenght_password as i32)).log2();
    println!("Char type - lower: {}, upper: {}, number: {}, symbol:{}",
             result_lower, result_upper, result_number, result_punct, );
    let mut entropy_value: ColoredString = entropy.to_string().red();
    let mut status: bool = false;
    if entropy >= 70.0_f64{
        entropy_value = entropy.to_string().green();
        status = true
    }
    println!("Password info - lenght_password: {}, dictionnary: {}, entropy: {}",
             lenght_password, dictionnary_count, entropy_value);

    status

}

pub(crate) async fn hibp_password(password: &str) -> Result<bool, SpasmError> {

    let pwned_password = hibp::pwned_passwords::pwned_passwords(password).await?;

    let mut value: ColoredString = "false".to_string().green();
    if pwned_password.pwned{
        value="true".to_string().red();
    }

    println!(
        "Pwned?: {} | Times: {}",
        value, pwned_password.times.to_string().red()
    );

    Ok(pwned_password.pwned)
}

pub(crate) fn message_for_good_password(good_password: bool){
    if good_password{
        let comments = "It's a good password :)".to_string().green();
        println!("{}", comments);
    } else {
        let comments = "Try to find a better password :(".to_string().red();
        println!("{}", comments);
    }
}

pub(crate) fn prompt_string(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    let mut string = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut string)?;
    Ok(string.trim().to_string())
}


pub(crate) async fn is_good_password(password: String) -> Result<bool, SpasmError>{
    let good_entropy: bool = entropy(&password);
    let mut good_hipb:bool = false;
    match hibp_password(password.as_str()).await{
        Ok(retour) => good_hipb=retour,
        _ => {}
    }
    let good_password: bool = good_entropy & !good_hipb;
    message_for_good_password(good_password);
    Ok(good_password)
}

pub fn to_clipboard(password: Vec<u8>) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(String::from_utf8(password).unwrap()).unwrap();
    println!("(Password is in clipboard)");
}

pub(crate) fn load_env_variables(){
    let mut path_absolute = env::current_exe().unwrap().display().to_string();
    path_absolute.truncate(path_absolute.chars().count()-5);
    let last_path= format!("{}{}",path_absolute,".env");
    dotenv::from_path(Path::new(&last_path)).ok();

}

pub(crate) fn print_all_env_variables(){
    let varrrss = env::vars().into_iter();
    for x in varrrss {
        println!("{} - {}", x.0,x.1);
    }
}

pub(crate) fn get_entries_path() -> PathBuf{
    // load path of passwords
    let path_string = vars::password_path().clone();
    let mut path_absolute = env::current_exe().unwrap().display().to_string();
    path_absolute.truncate(path_absolute.chars().count()-5);

    let full_path_string = format!("{}{}", path_absolute, path_string);
    Path::new(&full_path_string).to_path_buf()
}

pub(crate) fn get_dice_path()-> PathBuf{
    // load path of passwords
    let path_string = vars::diceware_file_path().clone();
    let mut path_absolute = env::current_exe().unwrap().display().to_string();
    path_absolute.truncate(path_absolute.chars().count()-5);

    let full_path_string = format!("{}{}", path_absolute, path_string);
    Path::new(&full_path_string).to_path_buf()
}