
#[cfg(test)]
pub mod test{
    #[test]
    fn result() {
        use super::*;

        let query = "duct";

        let contents = "\
            Rust:
            safe, fast, productive
            Pick three";

        let r = search(contents, query);
        assert_eq!(vec!["safe, fast, productive"], r);
    }

    #[test]
    fn sensitive() {
        use super::*;

        let query = "Rust";

        let contents = "\
            Rust:
            safe, fast, productive
            Pick three";

        let r = sensitive_search(contents, query);

        let mut arr = Vec::new();
        for l in r.into_iter() {
            arr.push(l.to_lowercase());
        }
        assert_eq!(vec!["rust:"], arr);
    }
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

pub fn sensitive_search<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
    let q = query.to_lowercase();

    let mut r = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&q) {
            r.push(line.trim());
        }
    }
    r
}

fn sensitive_search2<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| { line.to_lowercase().contains(query)}).collect()
}