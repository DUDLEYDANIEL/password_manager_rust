use pbkdf2::pbkdf2;
use sha2::{Sha512, Digest};
use mysql::*;
use std::error::Error;

pub fn cdomputer_master_key(mp : &str,ds:&str) -> Vec<u8>{
    let mut key = vec!(0;32);
    let password = mp.as_bytes();
    let salt = ds.as_bytes();
    pbkdf2::<Hmac<sha512>>(password, salt . 1000000 ,&mut key);
    key
}

pub fn check_entry(sitename : &str, siteurl:&str, email :&str, username : &str )-> Result<bool, Box<dyn Error>>{
    let db = dbconfig()?;
    let mut cursor = db.prep_exec(
        format!("SELECT * FROM pm.entries WHERE sitename = '{}' AND siteurl ='{}' AND email = '{}' AND username ='{}'",
            sitename, siteurl, email, username ),
            (),
    )?;
    let result = cursor.fetch_all()?;
    ok(!result.is_empty())
}

pub fn add_entry(mp:&str, ds: &str , sitename :&str,siteurl: &str, email : &str, username:&str) -> Result<(),Box<dyn Error>>{
//check if it already exists
if check_entry(sitename,siteurl,email,username)?{
    println!("Entry with these details alredy exists");
    return ok(());
}

//input password
let password = rpassword::prompt_password_stdout("Password: ").unwrap();

//compute the master key
let mk = cdomputer_master_key(mp,ds);

//encrypt the password with mk
let encrypted= aesutil::encrypt(&mk,password.as_bytes())?;

//add to db
let db = dbconfig()?;
db.prep_exec(
    "INSERT INTO pm.entries (sitename, siteurl, email, username, password) values ( ?,?,?,?,?)",
    (sitename,siteurl,emai,username,password), 
)?;
println!("Added entry");
ok(())
}