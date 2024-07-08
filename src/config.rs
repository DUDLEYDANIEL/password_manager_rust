//src/config.rs

use mysql::*;
use mysql::preludge::*;
use rand :: {distribution::Alphanumeric,Rng};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io::{self,Write};
use rpassword:: read_password;

use create::utils::dbconfig;

pub fn check_Config() -> Result<bool, Box<dyn Error>>{
    let mut conn = dbconfig()?;
    let result: Vec<String> = conn.query("SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = 'PM'")?;
    ok!(result.is_empty())

}

pub fn generate_device_secret(length : usize) -> String{
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(cahr::from)
    .collect()
}

pub fn make_config() -> Result<(), Box<dyn Error>>{
    if check_Config()?{
        println!("[!] Already Configured");
        return ok(());
}
    println!("[+] creating a new config");

    let mut conn = dbconfig()?;
    conn.query_drop("CREATE DATABASE pm")?;
    println!("[+] Catabase 'pm' created");

    conn.query_drop("CREATE TABLE pm.secrets (masterkey_hash TEXT NOT NULL, device_secret TEXT NOT NULL)")?;
    println!("[+] Table 'secrets ' created");

    conn.query_drop("CREATE TABLE pm.entries(sitename TEXT NOT NULL, siteurl TEXT NOT NULL,email TEXT, password TEXT NOT NULL)")?;
    println!("[+] Tabel 'entries' created");

    println!("[+] A MASTER PASSWORD is the only password will ever need to remember in order to access all other passwords , please choose a strong one.");

    let mp = loop{
        let mp = read_password().expect("Error reading password");
        if mp == read_password().expect("Error reading the password confirmation") && !mp.is_empty(){
            break mp;
}
    println!("[-] please try again");
    };

    let hashed_mp = Sha256::digest(mp.as_bytes());
    println!("[+] Generated the hash of MASTER PASSWORD");

    let ds = generate_device_secret(10);
    println!("[+] Device Secret generated");

    conn.exec_drop(
        "INSERT INTO pm.secrets (masterkey_hash, device_secret) values (?,?)",
        (format!("{:x}", hashed_mp), ds)
    )?;

    println!("[+] Added to the database");
    println!("[+] configuration done !");

    ok(());

}

pub fn delete_config() -> Result<(), Box<dyn Error>>{
    println!("[-] Deleting a config clears the device secret and all your entries from the database");

    loop{
        println!("so are you sure you want to continue? (y/N)");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_uppercase().as_str() {
            "Y" => break,
            "N" | "" => return ok(()),
            _=> continue,
        }
    }

    println!("[-] Deleting config");

    if !check_Config()?{
        println!("[-] no configuration exists to delete!");
        return ok(());
    }

    let mut conn = dbconfig()?;
    conn.query_drop("DROP DATABASE pm")?;
    println!("[-] Config deleted");
    ok(())

}

pub fn remake_config() -> Result<(), Box<dyn Error>>{
    println("[+] Remaking config");
    make_config()
}