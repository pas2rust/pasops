use tokio::io::AsyncWriteExt;
use super::prelude::*;

pub async fn update_badge(args: &Args) -> MyResult<()> {
    let count = args.count.unwrap_or(0);
    let mut badge_url = format!(
        "/badge/{}-{}-{}.svg",
        args.label.replace(" ", "%20"),
        count,
        args.color
    );
    let mut query_params = Vec::new();

    if let Some(logo) = &args.logo {
        query_params.push(("logo", logo.as_str()));
    }
    if let Some(logo_color) = &args.logo_color {
        query_params.push(("logoColor", logo_color.as_str()));
    }
    if !query_params.is_empty() {
        badge_url.push('?');
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        badge_url.push_str(&query_string);
    }

    let http = shields_http()?;
    let resp = http
        .get(&badge_url, Vec::<(&str, &str)>::new())
        .await
        .map_err(|e| format!("failed to fetch shields.io error: {}", e))?;
    let status = resp.status();

    let bytes = resp.bytes().await.map_err(|e| {
        format!("failed to read shields.io response body as bytes: {}", e)
    })?;

    if !status.is_success() {
        let txt = String::from_utf8_lossy(&bytes);
        return Err(format!("shields.io returned HTTP {}: {}", status, txt).into());
    }

    if bytes.is_empty() {
        return Err("shields.io returned an empty body".into());
    }

    let mut file = tokio::fs::File::create(&args.badge_name).await?;
    file.write_all(&bytes).await?;
    file.sync_all().await?;
    
    drop(file);
    git(args).await?;

    Ok(())
}
