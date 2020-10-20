use std::fs::File;

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
        let f=File::open("config.toml");
        let f= match f{
            Ok(ff)=>f::read_to_string(),
            _ => println!("Tt is some errors."),
        } ;

        unimplemented!()
    }
}

