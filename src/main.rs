use argh::FromArgs;
use rpassword::read_password;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use clipboard::{ClipboardContext, ClipboardProvider};
use mysql::*;

use utils::{add, retrieve, generate, dbconfig};

#[derive(FromArgs)]
/// Description
struct Args {

    name: Option<String>,
    /// Description for url field.
    url: Option<String>,
    /// Description for email field.
    email: Option<String>,
    /// Description for login field.
    login: Option<String>,
    /// Description for length field.
    length: Option<usize>,
    /// Description for copy flag.
    copy: bool,
}

fn input_and_validate_master_password() -> Option<(String, String)> {
    print!("[yellow][!] MASTER PASSWORD: [/yellow]");
    let mp = read_password().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(mp.as_bytes());
    let hashed_mp = format!("{:x}", hasher.finalize());
    
    let query = "SELECT * FROM pm.secrets";
    let result: Vec<(String, String)> = conn.query(query).unwrap();

    if let Some((stored_hashed_mp, ds)) = result.get(0) {
        if &hashed_mp != stored_hashed_mp {
            print!("[red][!] WRONG![/red]");
            return None;
        }
        return Some((mp, ds.clone()));
    }
    None
}

fn main() {
    let args: Args = argh::from_env();

    match args.Option.as_str() {
        "add" | "a" => {
            if args.name.is_none() || args.url.is_none() || args.login.is_none() {
                if args.name.is_none() {
                    print!("[red][!][/red] Site Name (-s) required");
                }
                if args.url.is_none() {
                    print!("[red][!][/red] Site URL (-u) required");
                }
                if args.login.is_none() {
                    print!("[red][!][/red] Site Login (-l) required");
                }
                return;
            }

            let email = args.email.unwrap_or_default();
            if let Some((mp, ds)) = input_and_validate_master_password() {
                add::add_entry(&mp, &ds, &args.name.unwrap(), &args.url.unwrap(), &email, &args.login.unwrap());
            }
        },
        "extract" | "e" => {
            if let Some((mp, ds)) = input_and_validate_master_password() {
                let mut search = HashMap::new();
                if let Some(name) = args.name.clone() {
                    search.insert("sitename", name);
                }
                if let Some(url) = args.url.clone() {
                    search.insert("siteurl", url);
                }
                if let Some(email) = args.email.clone() {
                    search.insert("email", email);
                }
                if let Some(login) = args.login.clone() {
                    search.insert("username", login);
                }
                retrieve::retrieve_entries(&mp, &ds, &search, args.copy);
            }
        }
        
    }
}