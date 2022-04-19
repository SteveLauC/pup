//! match.rs: offers `![](/)` match functionality

use regex::{Match, Regex};
use std::collections::HashMap;
use std::ops::{Index, Range};
use std::path::PathBuf;

/// type to represent a matched line
pub struct MatchedLine<'lifetime_of_line> {
    pub line: &'lifetime_of_line mut String,
    pub range: Vec<Range<usize>>, // the positions of the path on a line
    pub paths: Vec<PathBuf>,
}

impl<'lifetime_of_line> MatchedLine<'lifetime_of_line> {
    /// purpose: check and initialize a matched line struct
    ///
    /// arguments:
    ///     * `line`: reference to the matched line string
    pub fn new(line: &'lifetime_of_line mut String) -> Option<Self> {
        let mut range_vec: Vec<Range<usize>> = Vec::new();
        let mut path_vec: Vec<PathBuf>  = Vec::new();

        // used to match the markdown image path
        let md_image_re: Regex = Regex::new(r#"!\[.*\]\(.*\)"#).unwrap();

        if md_image_re.is_match(line) {
            // used to match the parenthesis
            let parenthesis_re: Regex = Regex::new(r#"\(.*\)"#).unwrap();

            for mth in md_image_re.find_iter(line) {
                let md_image: &str = line.index(mth.range());
                let parenthesis_mth: Match = parenthesis_re.find(md_image).unwrap();

                // calculate the range`[start, end)` of path in this line
                let start: usize = parenthesis_mth.start() + mth.start() + 1; // inclusive
                let end: usize = parenthesis_mth.end() + mth.start() - 1; // exclusive

                if line[start..end].starts_with("https://") || line[start..end].is_empty() {
                    continue;
                }
                range_vec.push(Range { start, end });
                path_vec.push(PathBuf::from(line.index(Range{start, end})));
            }

            if range_vec.is_empty() {
                return None;
            }

            Some(Self {
                line,
                range: range_vec,
                paths: path_vec,
            })
        } else {
            None
        }
    }

    /// purpose: replace the local path with the returned URL
    ///
    /// arguments:
    ///     * `url`: returned URLs
    pub fn replace(&'lifetime_of_line mut self, urls: HashMap<usize, &str>) {
        for (&idx, url) in urls.iter() {
            self.line.replace_range(self.range[idx].clone(), url);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn replace_test() {
//         let mut local_path: String = String::from("> ![title](/home/steve/doc.png)xx");
//         let mut mth: MatchedLine = MatchedLine::new(&mut local_path).unwrap();
//         let target_url: &str= "https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png";
//         mth.replace(HashMap::from([(0, target_url)]));
//         assert_eq!(local_path, "> ![title](https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png)xx");
//     }

//     #[test]
//     fn matched_line_init_test() {
//         let mut line: String = String::from("> ![title](/home/steve/doc.png)xx");
//         let mth: MatchedLine = MatchedLine::new(&mut line).unwrap();

//         assert_eq!(mth.range, vec![Range { start: 11, end: 30 }]);
//     }

//     #[test]
//     fn matched_test() {
//         // empty path
//         let mut line1: String = "![]() ![]()".into();
//         assert!(MatchedLine::new(&mut line1).is_none());

//         let mut line2: String = "![](/)".into();
//         assert!(MatchedLine::new(&mut line2).is_some());

//         let mut line3: String = "![aaa[()".into();
//         assert!(MatchedLine::new(&mut line3).is_none());
//         // url
//         let mut line4: String = "> 我们不是![ppt](https://.....)xxxx这样".into();
//         assert!(MatchedLine::new(&mut line4).is_none());
//         // relative path
//         let mut line5: String = "![issustration](pic.png)".into();
//         assert!(MatchedLine::new(&mut line5).is_some());

//         let mut line6: String = "![1](https://) ![](https://)".into();
//         assert!(MatchedLine::new(&mut line6).is_none());
//     }
// }
