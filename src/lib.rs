use serde::{Deserialize, Serialize};

/// Category of an APK entry based on its path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryCategory {
    Dex,
    Resource,
    NativeLib,
    Asset,
    Signature,
    Manifest,
    Other,
}

/// Summary statistics for an APK file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApkSummary {
    pub entry_count: usize,
    pub dex_count: usize,
    pub dex_total_size: u64,
    pub has_manifest: bool,
    pub native_abis: Vec<String>,
}

/// Categorize an APK entry by its path.
#[must_use]
pub fn categorize_entry(path: &str) -> EntryCategory {
    if path == "AndroidManifest.xml" {
        return EntryCategory::Manifest;
    }
    if path.ends_with(".dex") {
        return EntryCategory::Dex;
    }
    if path.starts_with("lib/") && path.ends_with(".so") {
        return EntryCategory::NativeLib;
    }
    if path.starts_with("assets/") {
        return EntryCategory::Asset;
    }
    if path.starts_with("res/") || path == "resources.arsc" {
        return EntryCategory::Resource;
    }
    if path.starts_with("META-INF/") {
        return EntryCategory::Signature;
    }
    EntryCategory::Other
}

/// Summarize a list of APK entries (path, size) into an `ApkSummary`.
#[must_use]
pub fn summarize_entries(entries: &[(String, u64)]) -> ApkSummary {
    let mut dex_count: usize = 0;
    let mut dex_total_size: u64 = 0;
    let mut has_manifest = false;
    let mut native_abis: Vec<String> = Vec::new();

    for (path, size) in entries {
        match categorize_entry(path) {
            EntryCategory::Dex => {
                dex_count += 1;
                dex_total_size += size;
            }
            EntryCategory::Manifest => {
                has_manifest = true;
            }
            EntryCategory::NativeLib => {
                // Extract ABI from lib/<abi>/libfoo.so
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() >= 2 {
                    let abi = parts[1].to_string();
                    if !native_abis.contains(&abi) {
                        native_abis.push(abi);
                    }
                }
            }
            _ => {}
        }
    }

    native_abis.sort();

    ApkSummary {
        entry_count: entries.len(),
        dex_count,
        dex_total_size,
        has_manifest,
        native_abis,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categorize_dex_entry() {
        assert_eq!(categorize_entry("classes.dex"), EntryCategory::Dex);
        assert_eq!(categorize_entry("classes2.dex"), EntryCategory::Dex);
    }

    #[test]
    fn categorize_native_lib_entry() {
        assert_eq!(
            categorize_entry("lib/arm64-v8a/libnative.so"),
            EntryCategory::NativeLib
        );
        assert_eq!(
            categorize_entry("lib/x86_64/libfoo.so"),
            EntryCategory::NativeLib
        );
    }

    #[test]
    fn categorize_manifest_entry() {
        assert_eq!(
            categorize_entry("AndroidManifest.xml"),
            EntryCategory::Manifest
        );
        // A manifest nested elsewhere is not the top-level manifest
        assert_ne!(
            categorize_entry("res/AndroidManifest.xml"),
            EntryCategory::Manifest
        );
    }

    #[test]
    fn summarize_mixed_entries() {
        let entries = vec![
            ("classes.dex".to_string(), 1000),
            ("classes2.dex".to_string(), 2000),
            ("AndroidManifest.xml".to_string(), 100),
            ("lib/arm64-v8a/libnative.so".to_string(), 500),
            ("lib/x86_64/libnative.so".to_string(), 400),
            ("res/layout/main.xml".to_string(), 50),
            ("assets/fonts/roboto.ttf".to_string(), 300),
            ("META-INF/CERT.RSA".to_string(), 200),
        ];

        let summary = summarize_entries(&entries);
        assert_eq!(summary.entry_count, 8);
        assert_eq!(summary.dex_count, 2);
        assert_eq!(summary.dex_total_size, 3000);
        assert!(summary.has_manifest);
        assert_eq!(summary.native_abis, vec!["arm64-v8a", "x86_64"]);
    }

    #[test]
    fn summarize_empty_entries() {
        let entries: Vec<(String, u64)> = vec![];
        let summary = summarize_entries(&entries);
        assert_eq!(summary.entry_count, 0);
        assert_eq!(summary.dex_count, 0);
        assert_eq!(summary.dex_total_size, 0);
        assert!(!summary.has_manifest);
        assert!(summary.native_abis.is_empty());
    }
}
