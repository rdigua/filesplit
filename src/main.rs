//mod config;
//mod conf;

//#![feature(split_inclusive)]
///To split Go play file (.sgf)
///Splitting  format "(; )"
/// Save director is ./files
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
//, ErrorKind};
use std::env;
use std::path::Path;
//use crate::conf::Config;
use filesplit::*;
use filesplit::conf::Config;
type E=String;
//use toml::Value;
//use clap::{Arg, App};
fn size_split(file_name:String,dir:String,len:String)->Result<String,E>{
    unimplemented!();
}
fn delimiter_split(file_name:String,dir:String,deli:String)->Result<String,E>{
    unimplemented!();
}
fn main() -> io::Result<()> {

    let mut delimiter:String=String::new();
    let mut optput:String=String::new();
    let mut method:String=String::new();
    let mut len:String=String::new();
    let mut env: Vec<String> = Vec::new();
    for argument in env::args() {
        env.push(argument);
    }
    if env.len() <= 1 {
        println!("It is not Input file name.");
//        Err("No input name");
        return Err(Error::last_os_error());
    }
    if Path::new("config.toml").exists() {
        let con: conf::Config = conf::read_toml_from_file("config.toml").unwrap();

        let toml_str=r#"
        title="Split file"
[dim]
left="(;"
right=''
[dir]
output="./files"
[splitting]
# length or  separator
method = "separator"
# b k m g
len="100k"#;

        let con: conf::Config = conf::read_toml_from_str(toml_str).unwrap();
        assert_eq!(con.title, Some("Split file".to_string()));
        assert_eq!(con.dim.left, Some("(;".to_string()));
        assert_eq!(con.dim.right, Some("".to_string()));
        assert_eq!(con.dir.output, Some("./files".to_string()));
        assert_eq!(con.splitting.method, Some("separator".to_string()));
        assert_eq!(con.splitting.len, Some("100k".to_string()));
    }
    /*
    let config=config::format::default();
    let mut delimiter:String=config.get();
    let mut buf:String=String::new();
    if Path::new("config.toml").exists(){
        let mut f = File::open("config.toml")?;
        f.read_to_string(&mut buf);
        println!("{}",buf);
    }

    for i in env.iter() {
        println!("{}", i);
    }
    if env.len() <= 1 {
        println!("It is not Input file name.");
//        Err("No input name");
        return Err(Error::last_os_error());
    }
    if !Path::new("./files").exists() {
        std::fs::create_dir("./files")?;
    }
    let mut key: u32 = 0;
    let f_name = &env[1];

    let mut f = File::open(f_name)?;
    let mut buffer: String = String::new();
    f.read_to_string(&mut buffer)?;
    //let mut v:Vec<&str>=Vec::new();

    //let v: Vec<&str> = buffer.split_inclusive("(;").collect();
    //let v: Vec<&str> = buffer.split("(;").collect();
    let v:Vec<&str>=buffer.split(&delimiter).collect();
    
	let name = f_name.replace(".txt", "-").to_string();
    for i in v.iter() {
        if i.is_empty() {
            continue;
        }
        key += 1;
		let mut all="(;".to_string();
		all.push_str(i);
        let new_file = "./files/".to_owned() + &name.to_owned() + &key.to_string() + ".sgf";
        println!("To creat {}", new_file);
        let mut f = File::create(new_file)?;
        f.write_all(all.as_bytes())?;
        f.sync_data()?;
    */

/*let matches =App::new("Jay file split")
    .version("0.0.1")
    .author("Jay<digua@163.com>")
    .arg(Arg::new("v")
        .short('v')
        .multiple(true)
        .about("version"))
    .arg(Arg::new("INPUT")
        .about("Sets the input file to use")
        .required(true)
        .index(1))
    .get_matches();

 */


// Prints each argument on a separate line

Ok(())
}
