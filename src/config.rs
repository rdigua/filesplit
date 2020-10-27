use toml;
use toml::de::Error;

struct Dlimiter {
    left:String,
    right:String,
}

impl Dlimiter{
	pub set(s:Vec<String>){
		
	}
}
pub struct F {
    pub delimiter: String,
}

impl F {
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
}

pub fn set_dlimiter(s:String)->String{
	    if Path::new("config.toml").exists(){
        let mut f = File::open("config.toml")?;
        f.read_to_string(&mut buf);
        println!("{}",buf);
    }
	let v:Vec<&str>=buffer.split("\n").collect();
	
	//let v: Vec<&str> = buffer.split_inclusive("(;")collect();
	let mut dlimiter="(;".to_string();
	let k:Result<Dlimiter, Error>= toml::from_str(&s);
	match k{
		Ok(d) => dlimiter=d.left,
		_ => dlimiter="(;".to_string(),
	};
	dlimiter
}
/*
left="(;"
right=""
 */
