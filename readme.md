# Spasm
```                          
  ____  _____      __      ____    ___ ___    
 /',__\/\ '__`\  /'__`\   /',__\ /' __` __`\  
/\__, `\ \ \L\ \/\ \L\.\_/\__, `\/\ \/\ \/\ \ 
\/\____/\ \ ,__/\ \__/.\_\/\____/\ \_\ \_\ \_\
 \/___/  \ \ \/  \/__/\/_/\/___/  \/_/\/_/\/_/
          \ \_\                               
           \/_/                               
```
{S}mol {PAS}sword {M}anager


## Goals
I was fedup to store all my passwords into commercial tools and to have no tools to generate or evaluate my passwords.

You'll find here some cli tools to:
- manage passwords
- generate password or passphrase (diceware)
- tools to check the quality of a password using Shannon entropy and leakage from Have I been pwnd?

The main goal is to have something simple to modify for your needs.

Be aware that I'm not responsible of the loss of your password if theres any bug. I use it every day and it's a week end project so there's no warranty. 

NB: The Master Key you use is never stored and can be different for each entry. No possibility to recover lost master key(s).

## Releases 
- Releases are available for linux based on stable-x86_64-unknown-linux-gnu (default) rustc 1.46.0 (04488afe3 2020-08-24)
- I'll try to do something on macosx
- If someone want to build on windows and test, be my guest.

## pre requisites on linux for clipboard
```
sudo apt-get install xclip upx
```

## build 
Install (rustlang)[https://www.rust-lang.org/learn/get-started]: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
and then build spasm

```shell script
cargo build --release
```
on linux you can add other cool stuffs to reduce the size
```
strip target/release/spasm
upx -9 target/release/spasm
```

Copy/Paste the spasm binary (from target/release) on a accessible Path from terminal (PATH variable for example).
Then copy bin/ directory with all the files in the same directory of spasm,
Finally copy .env file filed with the good properties near spasm

and start with a spasm -h

## Have I been Pwnd?
If you have an api key from (Have I been Pwnd?)[https://haveibeenpwned.com/API/Key]
You can use it by adding an env variable "HIBP_API" with the key (or you can add it into .env file - I removed it by default)

## Usage

    spasm -h : help and list all commands
    spasm list: list all entries
    spasm add: add a new entry
    spasm get: get your stored password back into your clipboard
    spasm del: delete an entry
    spasm generate: generate a password
    spasm dice: generate a passphrase based on diceware
    
    spasm -P <some_password> : check the quality of your password
    spasm -b <some_email> : check if there's some leaks with this email
    spasm --auto-completion <out> : generate autocompletion for out = bash or elvish or fish or powershell or zsh


## Diceware
It's based on diceware wordlist. I simplified those files.
http://weber.fi.eu.org/software/diceware/src/
https://github.com/yuvallanger/rusty-diceware

## Crypto
Each password entry is individually encrypted and authenticated with ChaCha20/Poly1305. 
The key is derived with Argon2id from the master key and a randomly generated 16-byte salt. 
The plaintext is the UTF-8 encoding of the password. 
The additionally associated data consists of the entry name, shortened name, and extra data. 
The exact construction is SHA256(name) || SHA256(short) || SHA256(extra) to avoid collisions.
All metadata (long name, short name and login/extra) are not encrypted. Try to open .spasm.json to check how it's stored. 

## Quality of a password
This is based on 2 criteria:
- Shannon's entropy. If it superior to 70, it's not perfect but it's a start for a good password it's totally subjective (today 12/10/2020). It depends off so many things like type of storage of password (ssha1,...), electical cost of an infrastructure versus a rig of cpu/gpu. You can do your math and try to define a nice threshold and recompile the app or add new features :)
  (https://tutorials.technology/blog/08-Hashcat-GPU-benchmarking-table-Nvidia-and-amd.html)
- Pwnd on Have I been pwnd (if you have set your api key in env variable HIBP_API into .env or into your workspace )

## Sources
I'm not a genius, I just mixed some source code from:
- https://github.com/defund/pw
- https://github.com/TypeError/pwnage.rs

Thanks for that's! (for the code not that I'm not a genius uwu)

## License
MIT

