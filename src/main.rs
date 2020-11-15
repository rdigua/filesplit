//use std::fs;
//use futures::join;
use std::fs::File;
use std::io::Read;
use std::env;
use std::path::Path;
use std::thread;
use std::io::Write;
use std::time::{Duration, Instant};
type E=String;

fn main() {
    let now = Instant::now();
    let mut v:Vec<String>=Vec::new();
    for argument in env::args() {
        v.push(argument);
        //println!("{}", argument);
    }
    if v.len()<2 {
        println!("No input file!");
    }
    for i in v.iter(){

        println!("{}", i);
    }

    //pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let mut filename:String=String::new();
    filename.push_str(&v[1]);

    let mut content:String=String::new();
    match read_file(filename.clone()) {
        Some(c)=>content.push_str(&c),
        _=> {},
    }
    if content.len()==0 {

        println!("No content in file.");
        return
    }

    let vv:Vec<String>=content.split("(;").map(|x|x.to_string()).collect();

    if vv.is_empty(){
        println!("Do not match split on Strings.");
        return
    }
    if !Path::new("./files").exists() {
        std::fs::create_dir("./files").expect("It cannot create ./files directory.")
    }
    let name = filename.replace(".txt", "-").to_string();
    let mut key: u32 = 0;
    //let all:u32=vv.len() as u32;
    let mut v_thread=vec![];
    for i in vv.iter(){
        key+=1;
        if i.is_empty(){
            continue
        }
        let new_file = "./files/".to_owned() + &name.to_owned() + &key.to_string() + ".sgf";
        let mut s:String="(;".to_string();
        s.push_str(i);

         let f_run=thread::spawn(move || {

            save_file(new_file,s);


        });
        v_thread.push(f_run);

    }

    for child in v_thread{
        child.join();
    }

    println!("{}", now.elapsed().subsec_nanos());
    //If It do not using thread, the "digua.txt" file will to be 64 second, else will be 15 second.

}

fn read_file<P: AsRef<Path>>(path: P) ->Option<String>{
    //let k= path::clone();
        let  f=File::open(path);

    let mut buffer = String::new();

    match f {
        Ok(mut file)=>{
           match file.read_to_string(&mut buffer){
               Ok(_size)=> return Some(buffer),
               _=> return None,
           }
        },
        _=> return None,
    }
    None
}

 fn save_file<P: AsRef<Path>>(path: P,s:String)->Result<String,E>{
     //println!("{}",path.as_ref().display());
     //let ss:String=String::from(path.as_ref().to_str().unwrap_or("It is empty?"));
    let  f = File::create(path);
    match f{
        Ok(mut ff)=>{
            if s.is_empty(){
                println!("file is not empty.");
                return Err("file is not empty.".to_string());
            }
           println!("{:#?}",ff.write_all(s.as_bytes()));
    //        ff.write_all(s.as_bytes());
            println!("{:#?}", ff.sync_data());
      //      ff.sync_data();
        //    println!("{}", ss);
        }
        _=> return Err("Is is some error, do not write file.".to_string())
    }

/*
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
    Ok("ok".to_string())
}
