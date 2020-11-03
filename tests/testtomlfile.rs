use filesplit::conf;

#[cfg(test)]
mod tests {
    use super::*;
/*
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

        let con: conf::Config = conf::read_toml("config").unwrap();
        assert_eq!(con.title, Some("Split file".to_string()));
        assert_eq!(con.dim.left, Some("(;".to_string()));
        assert_eq!(con.dim.right, Some("".to_string()));
        assert_eq!(con.dir.output, Some("./files".to_string()));
        assert_eq!(con.splitting.method, Some("separator".to_string()));
        assert_eq!(con.splitting.size, Some("100k".to_string()));

 //       let con: conf::Config = toml::from_str(toml_str).unwrap();
        let con: conf::Config = conf::read_toml_from_str(toml_str).unwrap();
        assert_eq!(con.title, Some("Split file".to_string()));
        assert_eq!(con.dim.left, Some("(;".to_string()));
        assert_eq!(con.dim.right, Some("".to_string()));
        assert_eq!(con.dir.output, Some("./files".to_string()));
        assert_eq!(con.splitting.method, Some("separator".to_string()));
        assert_eq!(con.splitting.size, Some("100k".to_string()));
    }

 */
    #[test]
    fn test_from_str() {
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

        //       let con: conf::Config = toml::from_str(toml_str).unwrap();
        let con: conf::Config = conf::read_toml_from_str(toml_str).unwrap();
        assert_eq!(con.title, Some("Split file".to_string()));
        assert_eq!(con.dim.left, Some("(;".to_string()));
        assert_eq!(con.dim.right, Some("".to_string()));
        assert_eq!(con.dir.output, Some("./files".to_string()));
        assert_eq!(con.splitting.method, Some("separator".to_string()));
        assert_eq!(con.splitting.size, Some("100k".to_string()));
    }
    #[test]
    fn test_from_file() {

        let con: conf::Config = conf::read_toml_from_file("config.toml").unwrap();
        assert_eq!(con.title, Some("Split file".to_string()));
        assert_eq!(con.dim.left, Some("(;".to_string()));
        assert_eq!(con.dim.right, Some("".to_string()));
        assert_eq!(con.dir.output, Some("./files".to_string()));
        assert_eq!(con.splitting.method, Some("separator".to_string()));
        assert_eq!(con.splitting.size, Some("100k".to_string()));

    }
    #[test]
    fn test_config_default() {
        let con:conf::Config;
        assert_eq!(con::Config{
            title: Some( "Split file".to_string()),
            dim: Dim{
                left:Some("(;".to_string()),
                right:Some("".to_string()),
            },
            dir:Dir{
                output:Some("./files".to_string()),
            },
            splitting:Splitting{
                method:Some("separator".to_string()),
                len:Some("100k".to_string()),
            },
        }, con.default())
    }

}