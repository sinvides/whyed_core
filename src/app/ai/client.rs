use crate::app::config::Config;
use reqwest::blocking::Client;
use serde_json::{json, Value};

pub struct ApiClient {
    client: Client,
    api_key: Option<String>,
    api_url: Option<String>,
}

impl ApiClient {
    pub fn from_config(config: &Config) -> Self {
        ApiClient {
            client: Client::new(),
            api_key: config.ai_key(),
            api_url: config.ai_url(),
        }
    }

    pub fn chat(&self, messages: &[Value]) -> Result<String, String> {
        let api_key = match &self.api_key {
            Some(k) => k.clone(),
            None => return Err("api ключ не настроен в секции [ai]".to_string()),
        };

        let api_url = match &self.api_url {
            Some(u) => u.clone(),
            None => return Err("api_url не настроен в секции [ai]".to_string()),
        };

        let url = if api_url.ends_with("/chat/completions") {
            api_url
        } else {
            format!("{}/chat/completions", api_url.trim_end_matches('/'))
        };

        let body = json!({
            "model": "llama",
            "messages": messages,
            "stream": false
        });

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?;

        let text = resp.text().map_err(|e| e.to_string())?;

        if let Ok(val) = serde_json::from_str::<Value>(&text) {
            if let Some(choices) = val.get("choices") {
                if let Some(first) = choices.get(0) {
                    if let Some(msg) = first.get("message") {
                        if let Some(content) = msg.get("content") {
                            if let Some(s) = content.as_str() {
                                return Ok(s.to_string());
                            }
                        }
                    }
                }
            }
            if let Some(error) = val.get("error") {
                if let Some(msg) = error.get("message") {
                    if let Some(s) = msg.as_str() {
                        return Err(format!("Ошибка API: {}", s));
                    }
                }
            }
        }

        Ok(text)
    }
}
