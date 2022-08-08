
#[cfg(test)]
mod tests{

    #[test]
    fn result() {
        let query = "duct";

        let contents = "\
            Rust:
            safe, fast, productive
            Pick three";

        let r = search(contents, query);
        assert_eq!(vec!["safe, fast, productive"], r);
    }

    pub fn search<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
        let mut r = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                r.push(line.trim())
            }
        }
        r
    }
}