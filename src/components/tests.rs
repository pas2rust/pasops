use super::prelude::*;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn count_tests_in_tests_folder() -> MyResult<u64> {
    let tests_dir = Path::new("./tests");

    fn inner(dir: &Path, re: &Regex) -> MyResult<u64> {
        let mut total: u64 = 0;

        for entry in fs::read_dir(dir).map_err(|e| {
            Box::<dyn Error + Send + Sync>::from(format!("read_dir {}: {}", dir.display(), e))
        })? {
            let entry = entry.map_err(|e| {
                Box::<dyn Error + Send + Sync>::from(format!(
                    "reading entry in {}: {}",
                    dir.display(),
                    e
                ))
            })?;
            let path = entry.path();

            if path.is_dir() {
                total = total.checked_add(inner(&path, re)?).ok_or_else(|| {
                    Box::<dyn Error + Send + Sync>::from("overflow counting tests")
                })?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let content = fs::read_to_string(&path).map_err(|e| {
                    Box::<dyn Error + Send + Sync>::from(format!(
                        "failed to read {}: {}",
                        path.display(),
                        e
                    ))
                })?;
                total += re.find_iter(&content).count() as u64;
            }
        }

        Ok(total)
    }

    if !tests_dir.exists() {
        return Ok(0);
    }

    let test_macro_re = Regex::new(r"#\[\s*(?:\w+:)?test\s*\]").unwrap();
    inner(tests_dir, &test_macro_re)
}
