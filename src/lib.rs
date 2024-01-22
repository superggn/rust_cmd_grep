use std::fs;

#[derive(Debug, PartialEq)]
pub struct Conf {
    ignore_case: bool,
    pattern: String,
    input_filepaths: Vec<String>,
}

impl Conf {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Conf, &'static str> {
        // 先把所有 flag 都 parse 掉， 然后再看剩下的参数够不够
        let mut args = args.skip(1);
        let mut ignore_case = false;
        let mut trailing_args: Vec<String> = vec![];
        let mut pattern = String::new();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" => ignore_case = true,
                "-f" => {
                    let pattern_file_path = match args.next() {
                        Some(path) => path,
                        None => "".to_owned(),
                    };
                    // extract pattern from file
                    pattern = match fs::read_to_string(&pattern_file_path) {
                        Ok(content) => content,
                        Err(_) => return Err("no pattern specified by -f flag!"),
                    };
                }
                _ => trailing_args.push(arg),
            }
        }
        if pattern == "".to_owned() {
            if trailing_args.len() == 0 {
                return Err("no pattern specified!");
            }
            pattern = trailing_args.pop().unwrap().to_owned();
        }
        let conf = Conf {
            ignore_case,
            pattern,
            input_filepaths: trailing_args,
        };
        // dbg!(&conf);
        Ok(conf)
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

fn search_ignore_case<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    let query_lower = query.to_lowercase();
    for line in content.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains(&query_lower) {
            res.push(line);
        };
    }
    res
}

fn search_hub(conf: &Conf, contents: &str) {
    let results = if !conf.ignore_case {
        // println!("distinct case");
        search(&conf.pattern, &contents)
    } else {
        // println!("ignore case");
        search_ignore_case(&conf.pattern, &contents)
    };
    println!("================================");
    // dbg!(&results);
    for line in results.iter() {
        println!("{}", line)
    }
}

pub fn run(conf: Conf) {
    // dbg!(&conf);
    if conf.input_filepaths.len() != 0 {
        for file_path in &conf.input_filepaths {
            let contents = fs::read_to_string(file_path).unwrap();
            search_hub(&conf, &contents)
        }
        return;
    }
    // let mut contents = String::new();
    let contents = std::io::read_to_string(std::io::stdin()).unwrap();
    // dbg!(&contents);
    search_hub(&conf, &contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build() {
        // 还要搞一下 demo file 的创建
        let args: String = "minigrep -i file_input1.txt file_input2.txt mypattern".to_owned();
        let iter = args.split_whitespace().map(|s| s.to_owned());
        let conf = Conf::build(iter).unwrap();
        dbg!(&conf);
        assert_eq!(
            conf,
            Conf {
                ignore_case: true,
                pattern: "mypattern".to_owned(),
                input_filepaths: vec!["file_input1.txt".to_owned(), "file_input2.txt".to_owned()],
            }
        );
    }

    #[test]
    fn test_search() {
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        // good match
        assert_eq!(search("safe", content), vec!["safe, fast, productive."]);
        // bad match
        assert_eq!(search("Safe", content), vec![] as Vec<&str>);
        println!("test_search")
    }

    #[test]
    fn test_search_ignore_case() {
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        // good match
        assert_eq!(
            search_ignore_case("safe", content),
            vec!["safe, fast, productive."]
        );
        assert_eq!(
            search_ignore_case("Safe", content),
            vec!["safe, fast, productive."]
        );
        // bad match
        assert_eq!(search_ignore_case("嗨害嗨", content), vec![] as Vec<&str>);
        println!("test_search")
    }
}
