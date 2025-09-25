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

    // Adicionar parâmetros de query
    let mut query_params = Vec::new();
    
    if let Some(logo) = &args.logo {
        query_params.push(("logo", logo.as_str()));
    }
    
    // Adicionar cor do logo se fornecida
    if let Some(logo_color) = &args.logo_color {
        query_params.push(("logoColor", logo_color.as_str()));
    }

    // Construir a string de query
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
    let svg = resp.text().await?;

    if !status.is_success() {
        return Err(format!("shields.io returned HTTP {}: {}", status, svg).into());
    }

    let mut file = tokio::fs::File::create(&args.badge_name).await?;
    file.write_all(svg.as_bytes()).await?;

    Ok(())
}