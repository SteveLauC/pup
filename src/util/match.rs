//! Offers `![](/)` match functionality in markdown manipulation

use regex::Regex;
use std::ops::{Index, Range};

/// type to represent a matched line
pub struct MatchedLine<'lifetime_of_line> {
    /// Line.
    pub line: &'lifetime_of_line mut String,
    /// The position of the path on a line.
    ///
    /// For example:
    /// "This is a pic ![pic](pic.jpeg)!"
    /// then `range` will be `Range {start: 21, end: 28 }`.
    pub range: Range<usize>,
}

impl<'lifetime_of_line> MatchedLine<'lifetime_of_line> {
    /// Check and initialize a matched line struct
    ///
    /// Return `Ok(MatchedLine<'_>)` if `line` contains a picture entry.
    pub fn new(line: &'lifetime_of_line mut String) -> Option<Self> {
        // used to match the markdown image path
        let image_path_re = Regex::new(r#"!\[.*\]\(.*\)"#).unwrap();

        if image_path_re.is_match(line) {
            let image_path_match = image_path_re.find(line).unwrap();

            // used to match the parenthesis
            let parenthesis_re = Regex::new(r#"\(.*\)"#).unwrap();
            let image_path = line.index(image_path_match.range());
            let parenthesis_mth = parenthesis_re.find(image_path).unwrap();

            // calculate the range`[start, end)` of path in this line
            let start = parenthesis_mth.start() + image_path_match.start() + 1; // inclusive
            let end = parenthesis_mth.end() + image_path_match.start() - 1; // exclusive

            if line[start..end].starts_with("https://") || line[start..end].is_empty() {
                None
            } else {
                Some(Self {
                    line,
                    range: Range { start, end },
                })
            }
        } else {
            None
        }
    }

    /// Replace the local path with the returned URL
    pub fn replace(&'lifetime_of_line mut self, url: &str) {
        self.line.replace_range(self.range.clone(), url);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn replace_test() {
        let mut local_path: String = String::from("> ![title](/home/steve/doc.png)xx");
        let mut mth: MatchedLine = MatchedLine::new(&mut local_path).unwrap();
        let target_url = "https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png";
        mth.replace(target_url);
        assert_eq!(local_path, "> ![title](https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png)xx");
    }

    #[test]
    fn matched_line_init_test() {
        let mut line: String = String::from("> ![title](/home/steve/doc.png)xx");
        let mth: MatchedLine = MatchedLine::new(&mut line).unwrap();

        assert_eq!(mth.range, Range { start: 11, end: 30 });
    }

    #[test]
    fn matched_test() {
        // empty path
        let mut line1: String = "![]()".into();
        assert!(MatchedLine::new(&mut line1).is_none());

        let mut line2: String = "![](/)".into();
        assert!(MatchedLine::new(&mut line2).is_some());

        let mut line3: String = "![aaa[()".into();
        assert!(MatchedLine::new(&mut line3).is_none());
        // url
        let mut line4: String = "> 我们不是![ppt](https://.....)xxxx这样".into();
        assert!(MatchedLine::new(&mut line4).is_none());
        // relative path
        let mut line5: String = "![issustration](pic.png)".into();
        assert!(MatchedLine::new(&mut line5).is_some());
    }
}
