use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub t: i64,
    pub total: f64,
    pub topped: f64,
    pub granted: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recharge {
    pub t: i64,
    pub amount: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub snapshots: Vec<Snapshot>,
    pub recharges: Vec<Recharge>,
}

pub fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

impl History {
    pub fn load(path: &Path) -> History {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(s) = serde_json::to_string(self) {
            let _ = std::fs::write(path, s);
        }
    }

    pub fn add_snapshot(&mut self, total: f64, topped: f64, granted: f64, currency: String) {
        let now = unix_now();
        if let Some(prev) = self.snapshots.last() {
            if topped - prev.topped > 0.000001 {
                self.recharges.push(Recharge {
                    t: now,
                    amount: (topped - prev.topped),
                });
            }
        }
        self.snapshots.push(Snapshot {
            t: now,
            total,
            topped,
            granted,
            currency,
        });
        if self.snapshots.len() > 5000 {
            let skip = self.snapshots.len() - 5000;
            self.snapshots.drain(0..skip);
        }
    }

    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.recharges.clear();
    }
}
