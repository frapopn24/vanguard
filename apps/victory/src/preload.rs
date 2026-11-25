//! Read-only status for the shader and host-pipeline preload caches.

use std::path::Path;

use anyhow::Result;

/// Show what the current configuration has already cached without changing it.
pub fn status() -> Result<()> {
    let cfg = victory_core::Config::load()?;
    let shader_dir = cfg.cache_dir.join("shaders");
    let pipeline_dir = cfg.cache_dir.join("pipelines");
    let pipeline_file = pipeline_dir.join("host-pipelines.bin");
    let early_file = pipeline_dir.join("early-pipelines.txt");
    let report_file = cfg.cache_dir.join("preload-report.txt");

    let (shader_files, shader_bytes) = tree_stats(&shader_dir);
    let pipeline_bytes = std::fs::metadata(&pipeline_file).ok().map(|m| m.len());
    let early = std::fs::read_to_string(&early_file).ok();
    let early_count = early.as_deref().map(|s| {
        let mut keys: Vec<_> = s.lines().map(str::trim).filter(|s| !s.is_empty()).collect();
        keys.sort_unstable();
        keys.dedup();
        keys.len()
    });
    let report_time = std::fs::metadata(&report_file).ok().and_then(|m| m.modified().ok());

    println!("VANGUARD PRELOAD STATUS");
    println!("cache root          {}", cfg.cache_dir.display());
    println!(
        "shader cache        {} files, {:.1} MiB{}",
        shader_files,
        shader_bytes as f64 / (1024.0 * 1024.0),
        if shader_dir.is_dir() { "" } else { " (not created yet)" }
    );
    match pipeline_bytes {
        Some(bytes) => println!(
            "D3D12 PSO library   {:.1} MiB ({})",
            bytes as f64 / (1024.0 * 1024.0),
            pipeline_file.display()
        ),
        None => println!("D3D12 PSO library   not created yet"),
    }
    match early_count {
        Some(n) => println!("learned PSO keys    {n} ({})", early_file.display()),
        None => println!("learned PSO keys    no history yet"),
    }
    if let Some(time) = report_time {
        println!("last preload report {}", format_time(time));
    } else {
        println!("last preload report none");
    }

    println!();
    println!("What these caches do:");
    println!("  - translated vertex, pixel, mesh and compute shaders are reused across runs");
    println!("  - compute PSOs can be built offline when their shader containers are found");
    println!("  - graphics PSOs are keyed by their complete runtime state; learned keys are");
    println!("    started early when the title recreates those matching pipeline objects");
    println!();
    println!("A shader-only scan cannot safely invent a complete graphics PSO: render-target");
    println!("formats, root signature and fixed-function state must match the title.");
    Ok(())
}

fn tree_stats(root: &Path) -> (usize, u64) {
    if !root.is_dir() {
        return (0, 0);
    }
    let mut files = 0usize;
    let mut bytes = 0u64;
    for entry in walkdir::WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            files += 1;
            bytes = bytes.saturating_add(entry.metadata().map_or(0, |m| m.len()));
        }
    }
    (files, bytes)
}

fn format_time(time: std::time::SystemTime) -> String {
    match time.duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => format!("unix:{}s", d.as_secs()),
        Err(_) => "unknown".to_owned(),
    }
}
