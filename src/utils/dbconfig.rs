// src/utils/dbconfig.rs

use mysql::*;
use mysql::prelude::*;
use std::error::Error;

pub fn dbconfig() -> Result<PooledConn,Box<dyn Error>>{
    let url = "mysql://pm:password@localhost:3306";
    let pool = Pool::new(url)?;
    let conn= pool.get_conn()?;

    println!("[+] connected to db");
    Ok(conn)
}
