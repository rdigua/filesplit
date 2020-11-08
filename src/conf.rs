//use std::io;
use std::io::prelude::*;
//use std::fs;
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::result::Result;
use super::fns;
//use std::error::Error;
type E = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: Option<String>,
    pub dim: Dim,
    pub dir: Dir,
    pub splitting: Splitting,
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
    pub len: Option<String>,
}

impl Config {
    fn set(&self) -> Self {
        unimplemented!()
    }
    fn get_from_file(&self, s: &str) -> Self {
        unimplemented!()
    }
    pub fn default() -> Self {
        Config {
            title: Some("Split file".to_string()),
            dim: Dim {
                left: Some("(;".to_string()),
                right: Some("".to_string()),
            },
            dir: Dir {
                output: Some("./files".to_string()),
            },
            splitting: Splitting {
                method: Some("separator".to_string()),
                len: Some("100k".to_string()),
            },
        }
    }
}

/*
    title = "Split file"
    [dim]
    left="(;"
    right=''
    [dir]
    output="./files"
    [splitting]
    method = "separator"
    size="100k"
*/


pub fn read_toml_from_file(s: &str) -> Result<Config, E> {
    let f = File::open(s);
    let mut buffer = String::new();
    let mut ss: String = String::new();
    if let Ok(mut ff) = f {
        match ff.read_to_string(&mut buffer) {
            Ok(_n) => ss = buffer,
            Err(_e) => return Err("It is some error".to_string()),
        }
        //ff.read_to_string(&mut buffer);

        //let con: Config = toml::from_str(&ss).unwrap();
        //return Ok(con);
        //Ok(())
    }
    match toml::from_str(&ss) {
        Ok(c) => return Ok(c),
        Err(e) => return Err(e.to_string()),
    };
    //f.read_to_string(&mut buffer)?;
    Err("It is some error".to_string())
    //Ok(con)
}

pub fn read_toml_from_str(s: &str) -> Result<Config, E> {
//    Result<T, Error>

    match toml::from_str(&s) {
        Ok(c) => return Ok(c),
        Err(e) => return Err(e.to_string()),
    };
    //let con: Config = toml::from_str(&s).unwrap();
    //    return Ok(con);
    //    Err("It is some error".to_string())
}
