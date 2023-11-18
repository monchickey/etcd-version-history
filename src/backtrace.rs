use core::fmt;

use etcd_client::{self, GetOptions};
use base64::{Engine as _, engine::general_purpose};

pub struct HistoryVersion {
    value: String,
    create_revision: i64,
    mod_revision: i64,
}

impl HistoryVersion {
    fn new() -> Self {
        HistoryVersion { value: String::new(), create_revision: 0, mod_revision: 0 }
    }

    pub fn get_create_version(&self) -> i64 {
        self.create_revision
    }

    fn from_kv(&mut self, kv: &etcd_client::KeyValue) {
        let value = kv.value_str();
        if let Ok(v) = value {
            self.value = v.to_string();
        } else {
            self.value = general_purpose::STANDARD.encode(kv.value());
        }
        self.create_revision = kv.create_revision();
        self.mod_revision = kv.mod_revision();
    }
}

impl fmt::Display for HistoryVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value: {} mod version: {}", 
            self.value, self.mod_revision)
    }
}

pub async fn tracking<'a>(client: &mut etcd_client::Client, key: &str) -> Result<Vec<HistoryVersion>, String> {
    let resp = client.get(key, None).await;
    if let Err(err) = resp {
        return Err(err.to_string());
    }
    let resp = resp.unwrap();
    let kv = resp.kvs().first();
    if let None = kv {
        return Err(format!("key {} does not exist", key));
    }
    let kv = kv.unwrap();

    let mut hvs = Vec::with_capacity(kv.version() as usize);

    let mut hv = HistoryVersion::new();
    hv.from_kv(kv);

    let (mut create_version, mut mod_version) = (hv.create_revision, hv.mod_revision);

    hvs.push(hv);

    while mod_version > create_version {
        let get_options = GetOptions::new().with_revision(mod_version - 1);
        let resp = client.get(key, Some(get_options)).await;
        if let Err(err) = resp {
            return Err(err.to_string());
        }
        let resp = resp.unwrap();
        let kv = resp.kvs().first();
        if let None = kv {
            break
        }
        let kv = kv.unwrap();
        let mut hv = HistoryVersion::new();
        hv.from_kv(kv);
        (create_version, mod_version) = (hv.create_revision, hv.mod_revision);
        hvs.push(hv);
    }
    hvs.reverse();
    return Ok(hvs);
}
