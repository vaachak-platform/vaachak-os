use heapless::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookFingerprint {
    pub algo: FingerprintAlgo,
    pub hex: String<64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FingerprintAlgo {
    Sha256Pathless,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderProgress {
    pub fingerprint: BookFingerprint,
    pub chapter: u16,
    pub page: u16,
    pub percentage_x100: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReaderBookmark {
    pub fingerprint: BookFingerprint,
    pub chapter: u16,
    pub page: u16,
    pub label: String<64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachePaths {
    pub root: String<96>,
    pub content_dir: String<96>,
}

impl CachePaths {
    pub fn from_fingerprint(fp: &BookFingerprint) -> Self {
        let mut root = String::new();
        let _ = root.push_str(".vaachakos/");
        let _ = root.push_str(fp.hex.as_str());

        let mut content_dir = String::new();
        let _ = content_dir.push_str(root.as_str());
        let _ = content_dir.push_str("/sections");

        Self { root, content_dir }
    }
}
