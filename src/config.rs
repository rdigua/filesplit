use std::fs::File;
use std::collections::HashMap;

pub struct format {
    pub delimiter: String,
}

impl format {
    pub fn default() -> Self {
        Self {
            delimiter: "(;".to_string(),
        }
    }
    pub fn set(s: String) -> Self {
        Self {
            delimiter: s,
        }
    }
    pub fn get(&self) -> String {
        let r = &self.delimiter;
        r.to_string()
    }

    pub fn from_file()->Self{
        let mut buf:String=String::new();
        let mut h:HashMap<String,String>=HashMap.new();
        let f=File::open("config.toml");
        let f= match f{
            Ok(ff)=>f::read_to_string(),
            _ => println!("Tt is some errors."),
        } ;
    let v:Vec<&str>=buf.split("\n").collect();
        for i in v.iter(){
            let s:String=i.chars().filter_map(|x| !x.split_whitespace()).collect();
            unimplemented!();
        }
        unimplemented!()
    }
}

fn vec_string_to_hushmap(s:String)->Result<HashMap<String,String>>{
    unimplemented!();
}
