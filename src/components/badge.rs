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

    if let Some(logo) = &args.logo {
        badge_url.push_str(&format!("?logo={}", logo));
    }

    let http = shields_http()?;
    let resp = http
        .get(&badge_url, Vec::<(&str, &str)>::new())
        .await
        .map_err(|e| format!("failed to fetch shields.io error: {}", e))?;
    let status = resp.status();
    let svg = resp.text().await?;

    if !status.is_success() {
        return Err(format!("shields.io returned HTTP {}: {}", status, svg).into());
    }

    let mut file = tokio::fs::File::create(&args.badge_name).await?;
    file.write_all(svg.as_bytes()).await?;

    Ok(())
}
