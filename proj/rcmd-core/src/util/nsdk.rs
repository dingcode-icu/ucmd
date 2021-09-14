use std::collections::HashMap;

pub async fn Get(uri: &str) -> Result<HashMap<String, String>, reqwest::Error>{
    Ok(reqwest::get(uri).await?
        .json::<HashMap<String, String>>()
        .await?)
}

