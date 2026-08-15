use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceInfo {
    pub currency: String,
    pub total: f64,
    pub topped: f64,
    pub granted: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResult {
    pub success: bool,
    pub is_available: bool,
    pub infos: Vec<BalanceInfo>,
    pub error: String,
    pub raw: String,
}

fn parse_num(v: Option<&serde_json::Value>) -> f64 {
    v.and_then(|x| {
        if let Some(n) = x.as_f64() {
            return Some(n);
        }
        if let Some(s) = x.as_str() {
            return s.parse::<f64>().ok();
        }
        None
    })
    .unwrap_or(0.0)
}

pub fn fetch_balance(api_key: &str) -> BalanceResult {
    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return BalanceResult {
                success: false,
                is_available: false,
                infos: vec![],
                error: e.to_string(),
                raw: String::new(),
            }
        }
    };

    let resp = client
        .get("https://api.deepseek.com/user/balance")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .send();

    match resp {
        Ok(r) => {
            let status = r.status();
            let raw = r.text().unwrap_or_default();
            if !status.is_success() {
                return BalanceResult {
                    success: false,
                    is_available: false,
                    infos: vec![],
                    error: format!("HTTP {}", status.as_u16()),
                    raw,
                };
            }
            match serde_json::from_str::<serde_json::Value>(&raw) {
                Ok(v) => {
                    let is_available = v
                        .get("is_available")
                        .and_then(|x| x.as_bool())
                        .unwrap_or(false);
                    let mut infos = Vec::new();
                    if let Some(arr) = v.get("balance_infos").and_then(|x| x.as_array()) {
                        for item in arr {
                            infos.push(BalanceInfo {
                                currency: item
                                    .get("currency")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("")
                                    .to_string(),
                                total: parse_num(item.get("total_balance")),
                                topped: parse_num(item.get("topped_up_balance")),
                                granted: parse_num(item.get("granted_balance")),
                            });
                        }
                    }
                    BalanceResult {
                        success: true,
                        is_available,
                        infos,
                        error: String::new(),
                        raw,
                    }
                }
                Err(e) => BalanceResult {
                    success: false,
                    is_available: false,
                    infos: vec![],
                    error: e.to_string(),
                    raw,
                },
            }
        }
        Err(e) => BalanceResult {
            success: false,
            is_available: false,
            infos: vec![],
            error: e.to_string(),
            raw: String::new(),
        },
    }
}
