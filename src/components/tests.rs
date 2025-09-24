use regex::Regex;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use async_recursion::async_recursion;

pub async fn count_tests_in_tests_folder() -> Result<u64, Box<dyn Error + Send + Sync>> {
    let tests_dir = Path::new("./tests");

    #[async_recursion]
    async fn inner(dir: &Path, re: &Regex) -> Result<u64, Box<dyn Error + Send + Sync>> {
        let mut total: u64 = 0;

        let mut entries = fs::read_dir(dir).await.map_err(|e| {
            Box::<dyn Error + Send + Sync>::from(format!("read_dir {}: {}", dir.display(), e))
        })?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                total = total.checked_add(inner(&path, re).await?).ok_or_else(|| {
                    Box::<dyn Error + Send + Sync>::from("overflow counting tests")
                })?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let content = fs::read_to_string(&path).await.map_err(|e| {
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
    inner(tests_dir, &test_macro_re).await
}
