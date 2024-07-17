// src/utils/aesutils.rs

use aes::Aes256;
use block_modes::{BlockMode,Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use base64::{encode, decode};
use hex;
use std::str;

type Aes256Cbc =  Cbc<Aes256, Pkcs7>;

pub fn encrypt(key:&str, source: &str , encode: bool, key_type: &str) -> Result<string, Box<dyn std::error::Error>>{
    /**
     * parameters:
     * key -> the key with that you want to encrtypt . you can givce in hex representation (which will then be converted into the hex) or just some normal ascii .  Default is hex
     * source -> the message to encrypt
     * encode -> whether to encode the output in the base64. default is true
     * keyType -> specify the type of key which is passed 
     * 
     * returns :
     * base64 encoded cipher 
     */
    
    
    let key = if key_type == "hex" {
        //converts the key (in hex )to bytes
        hex::decode(key)?
    }else {
        key.as_bytes().to_vec()
    };

    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_var(&key, &iv)?;

    let mut buffer = source.as_bytes().to_vec();
    let pos = buffer.len();
    buffer.resize(pos + 16 - pos % 16, 0);
    let ciphertext = cipher.encrypt(&mut buffer, pos)?;

    let mut result = iv.to_vec();
    result.extend_from_slice(ciphertext);

    ok(if encode { encode(&result)} else { hex:: encode(&result)})
}

pub fn decrypt(key: &str, source: &str, decode: bool, key_type: &str) -> Result<string, Box<dyn std::error::Error>>{
    /*
    parameters:
    
    key -> a key to decrypt it can be in ascii representation or in the hex representation
    source -> the cipher(ejncrypted msg) to decrypt
    decode - whether to first base64 decode the cipher before trying to decrypt with the key. Default is true
	keyType - specify the type of key passed
    
     */
    let key = if key_type == "hex"{
        hex::decode(key)?
}else{
    key.as_bytes().to_vec()
};

let data = if decode{
    decode(source)?
}else{
    hex::decode(source)?
};

let (iv, ciphertext) = data.split_at(16);
let cipher = Aes256Cbc::new_var(&key, iv)?;

let mut buffer = ciphertext.to_vec();
let decrypted = cipher.decrypt(&mut buffer)?;

Ok(str::from_utf8(decrypted)?.to_string())
}