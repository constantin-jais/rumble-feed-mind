use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use feedmind_sync::curated::CuratedItemExport;

const MAX_EXPORT_BYTES: u64 = 512 * 1024;

fn main() {
    println!("cargo:rerun-if-env-changed=FEED_RADAR_REVIEW_EXPORT");
    println!("cargo:rerun-if-env-changed=FEED_RADAR_REQUIRE_LIVE_EXPORT");

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let configured = env::var_os("FEED_RADAR_REVIEW_EXPORT").map(PathBuf::from);
    let require_live = env::var("FEED_RADAR_REQUIRE_LIVE_EXPORT").as_deref() == Ok("1");
    if require_live && configured.is_none() {
        panic!("FEED_RADAR_REVIEW_EXPORT is required for a live-sync proof build");
    }

    let (source, mode) = match configured {
        Some(path) => (absolute(&manifest, path), "live-sync"),
        None => (
            manifest.join("../../examples/expected-curated-export.json"),
            "fixture",
        ),
    };
    println!("cargo:rerun-if-changed={}", source.display());

    let metadata = fs::metadata(&source).expect("review export must exist");
    assert!(
        metadata.is_file() && metadata.len() <= MAX_EXPORT_BYTES,
        "review export must be a regular file no larger than {MAX_EXPORT_BYTES} bytes"
    );
    let raw = fs::read_to_string(&source).expect("review export must be UTF-8 JSON");
    let export: CuratedItemExport =
        serde_json::from_str(&raw).expect("review export must match CuratedItemExport");
    export
        .validate_client_safe()
        .expect("review export must pass client-safe validation");

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out.join("review-export.json"), raw).expect("copy reviewed export");
    fs::write(out.join("review-mode.txt"), mode).expect("write review mode");
}

fn absolute(manifest: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        manifest.join(path)
    }
}
