use super::prelude::*;
use neto::serde_json::Value;

pub async fn run_total_crates_io_downloads(user_login: &str) -> MyResult<u64> {
    let http = crates_http()?;

    let user_resp = http
        .get(&format!("/users/{user_login}"), Vec::<(&str, &str)>::new())
        .await
        .map_err(|e| format!("failed to fetch user: {e}"))?;

    let user_json: Value = user_resp
        .json()
        .await
        .map_err(|e| format!("failed to parse user JSON: {e}"))?;

    let user_id = user_json
        .get("user")
        .and_then(|u| u.get("id"))
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "missing user.id".to_string())?;

    let mut total: u64 = 0;
    let mut page = 1;
    let per_page = 100;

    loop {
        let resp = http
            .get(
                &format!("/crates?per_page={per_page}&page={page}&user_id={user_id}"),
                Vec::<(&str, &str)>::new(),
            )
            .await
            .map_err(|e| format!("failed to fetch crates page {page}: {e}"))?;

        let json: Value = resp
            .json()
            .await
            .map_err(|e| format!("failed to parse crates JSON: {e}"))?;

        let crates_arr = json
            .get("crates")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "missing crates array".to_string())?;

        if crates_arr.is_empty() {
            break;
        }

        for c in crates_arr {
            if let Some(dl) = c.get("downloads").and_then(|v| v.as_u64()) {
                total += dl;
            }
        }

        if crates_arr.len() < per_page as usize {
            break;
        }
        page += 1;
    }

    Ok(total)
}
