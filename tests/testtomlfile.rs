use filesplit::conf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let toml_str = r#"
    title = "Split file"
    [dim]
    left="(;"
    right=''
    [dir]
    output="./files"
    [splitting]
    method = "separator"
    size="100k"
    "#;

        let con: conf::Config = toml::from_str(toml_str).unwrap();
        assert_eq!(con.title, Some("Split file".to_string()));
        assert_eq!(con.dim.left, Some("(;".to_string()));
        assert_eq!(con.dim.right, Some("".to_string()));
        assert_eq!(con.dir.output, Some("./files".to_string()));
        assert_eq!(con.splitting.method, Some("separator".to_string()));
        assert_eq!(con.splitting.size, Some("100k".to_string()));
    }
}