use std::time::Duration;

const JIKAN_API: &str = "https://api.jikan.moe/v4";

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("luci-app/1.0")
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())
}

/// Total episode count for a MAL id (0 when unknown).
pub async fn episode_count(mal_id: i64) -> Result<i64, String> {
    let url = format!("{JIKAN_API}/anime/{mal_id}");
    let resp = client()?.get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("jikan returned HTTP {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(json.pointer("/data/episodes").and_then(|v| v.as_i64()).unwrap_or(0))
}
