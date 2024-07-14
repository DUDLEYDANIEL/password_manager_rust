use mysql::*;
use pbkdf2::pbkdf2;
use sha2::{Sha512, Digest};
use std::error::Error;

fn compute_master_key(mp:&str,ds: &str)->Vec<u8> {
    let password = mp.as_bytes();
    let salt = ds.as_bytes();
    let mut key = vec![0u8;32];

    pbkdf2::<Hmac<Sha512>>(password, salt , 1000000 , &mut key);

    key
}

fn retrieve_entries(mp : &str, ds: &str, search: &str, decrypt_password: bool) -> Result<(), Box<dyn Error>>{
    let db = dbconfig()?;

    let query = if search.is_empty() {
        ("SELECT * FROM pm.entries WHERE {}", search)
    };

    let mut stmt = db.prepare(query)?;
    let result = stmyt.execute(());

    let mut table = Table::new();
    table.add_column("Site Name");
    table.add_column("URL");
    table.add_column("Email");
    table.add_column("Username");
    table.add_column("Password");

    for row in result{
        let mut row = row?;
        let sitename: string = row.take(0);
        let siteurl: string = row.take(1);
        let email: string = row.take(2);
        let username: string= row.take(3);
        let encrypted_password: Vec<u8> = row.take(4);

        if decrypt_password && result.len() == 1{
            let master_key = compute_master_key(mp, ds);
            let decrypt_password = aesutil::decrypt(&master_key, &encrypted_password);
            printlnc!("[green][+][/green] Password copied toi clipboard");
        } else{
            table.add_row((sitename, siteurl, email, username,"{hidden}"));
        }
        printlnc!("{}",table);
        ok(())
    }

}