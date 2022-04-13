use anyhow::Result;
use regex::Regex;
use std::ops::{Index, Range};

pub fn replace(local_path: &mut String, url: &str) -> Result<()> {
    let outer_re: Regex = Regex::new(r#"!\[.*\]\(/.*\)"#)?;
    let innerr_re: Regex = Regex::new(r#"\(.*\)"#).unwrap();
    let local_path_clone: String = local_path.clone();

    if let Some(outer_mth) = outer_re.find(&local_path_clone) {
        let rela_str: &str = local_path.index(outer_mth.range());
        if let Some(inner_mth) = innerr_re.find(rela_str) {
            let start: usize = inner_mth.start() + outer_mth.start() + 1;
            let end: usize = inner_mth.end() + outer_mth.start() - 1;

            local_path.replace_range(Range { start, end }, url);
        }
    }
    Ok(())
}

mod test {
    use super::*;
    #[test]
    fn replace_test() {
        let mut local_path: String = String::from("> ![title](/home/steve/doc.png)xx");
        let target_url = "https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png";
        replace(&mut local_path, target_url).unwrap();
        assert_eq!(local_path, "> ![title](https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png)xx");
    }
}
