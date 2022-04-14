use anyhow::Result;
use regex::{Match, Regex};
use std::ops::{Index, Range};

pub struct MatchedLine<'a> {
    pub line: &'a mut String,
    pub range: Range<usize>,
}

impl<'a> MatchedLine<'a> {
    pub fn new(line: &'a mut String) -> Self {
        let outer_re: Regex = Regex::new(r#"!\[.*\]\(/.*\)"#).unwrap();
        let inner_re: Regex = Regex::new(r#"\(.*\)"#).unwrap();
        let line_clone: String = line.clone();

        let outer_mth: Match = outer_re.find(&line_clone).unwrap();
        let rela_str: &str = line_clone.index(outer_mth.range());
        let inner_mth: Match = inner_re.find(rela_str).unwrap();

        Self {
            line,
            range: Range {
                start: inner_mth.start() + outer_mth.start() + 1,
                end: inner_mth.end() + outer_mth.start() - 1,
            },
        }
    }

    pub fn replace(&mut self, url: &str) {
        self.line.replace_range(self.range.clone(), url);
    }
}

pub fn is_matched(line: &str) -> bool {
    let re: Regex = Regex::new(r#"!\[.*\]\(/.*\)"#).unwrap();
    re.is_match(line)
}

mod test {
    use super::*;
    #[test]
    fn replace_test() {
        let mut local_path: String = String::from("> ![title](/home/steve/doc.png)xx");
        let mut mth: MatchedLine = MatchedLine::new(&mut local_path);
        let target_url = "https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png";
        mth.replace(target_url);
        assert_eq!(local_path, "> ![title](https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png)xx");
    }

    #[test]
    fn matched_line_init_test() {
        let mut line: String = String::from("> ![title](/home/steve/doc.png)xx");
        let mth: MatchedLine = MatchedLine::new(&mut line);

        assert_eq!(mth.range, Range { start: 11, end: 30 });
    }

    #[test]
    fn matched_test() {
        let line1: &str = "![]()";
        assert_eq!(is_matched(line1), false);
        let line2: &str = "![](/)";
        assert_eq!(is_matched(line2), true);
        let line3: &str = "![aaa[()";
        assert_eq!(is_matched(line3), false);
        let line4: &str = "> 我们不是![ppt](https://.....)xxxx这样";
        assert_eq!(is_matched(line4), false);
        let line5: &str = "![issustratino]（/User/steve/...zz.md)";
        assert_eq!(is_matched(line5), false);
    }
}
