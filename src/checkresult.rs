use std::io;


fn get_str() -> std::result::Result<(), io::Result<()>> {
    fn get_string() -> io::Result<String> {
        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer)?;

        Ok(buffer)
    }
    let s = fn get_string();
    s
}