//use std::io;
use std::io::prelude::*;
//use std::fs;
use std::fs::File;
use serde::{Serialize, Deserialize};
//use std::error::Error;
type E=String;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: Option<String>,
    pub dim: Dim,
    pub dir: Dir,
    pub splitting:Splitting,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dim {
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dir {
    pub output: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Splitting {
    pub method: Option<String>,
    pub size: Option<String>,
}
pub fn read_toml(s:&str)->Result<Config,E>{
    let f = File::open(s);
    let mut buffer = String::new();
    if let Ok(mut ff)=f {
        ff.read_to_string(&mut buffer);
        let con: Config = toml::from_str(&buffer).unwrap();
        return Ok(con);
    }
    //f.read_to_string(&mut buffer)?;
    Err("It is some error".to_string())
    //Ok(con)
}

