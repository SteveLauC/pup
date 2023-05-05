use std::{env::set_current_dir, fs::canonicalize, path::Path};

/// Change current working directory to the parent directory of the markdown doc
/// so we can handle relative path.
#[inline]
pub fn adjust_pwd(target_markdown_file_path: &Path) {
    let md_absolute_path = canonicalize(target_markdown_file_path)
        .expect("Failed to get absolute path of target markdown file");
    let md_file_parent_dir = md_absolute_path
        .parent()
        .expect("The target Markdown doc should have a parent directory");
    set_current_dir(md_file_parent_dir)
        .expect("Failed to set current dir to the parent of the markdown doc");
}
