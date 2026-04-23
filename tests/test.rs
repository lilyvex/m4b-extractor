use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper to check MP3 metadata for a given file
fn check_mp3_metadata(mp3_path: &str, expected_title: &str, expected_track: u32) -> Result<()> {
    // Use ffprobe to get JSON metadata
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format_tags=title,track",
            "-print_format",
            "json",
            mp3_path,
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("ffprobe failed for '{}'", mp3_path);
    }

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let tags = &json["format"]["tags"];

    let title = tags["title"].as_str().unwrap_or("");
    let track = tags["track"]
        .as_str()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    assert_eq!(title, expected_title, "Title mismatch for '{}'", mp3_path);
    assert_eq!(track, expected_track, "Track mismatch for '{}'", mp3_path);

    Ok(())
}

#[test]
fn test_m4b_extractor_metadata() -> Result<()> {
    let binary_path = if cfg!(debug_assertions) {
        "./target/debug/m4b-extractor"
    } else {
        "./target/release/m4b-extractor"
    };

    let input_file = "tests/data/sample.m4b";
    let output_dir = "tests/output";

    // Clean previous test output
    if Path::new(output_dir).exists() {
        fs::remove_dir_all(output_dir)?;
    }

    // Run the binary
    let output = Command::new(binary_path)
        .args(&[input_file, "--output", output_dir])
        .output()?;

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "Binary did not exit successfully");

    // Check at least one chapter file exists
    let mut chapter_files: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "mp3")
                .unwrap_or(false)
        })
        .collect();

    // Sort by filename
    chapter_files.sort_by_key(|e| e.path().file_name().map(|f| f.to_os_string()));

    assert!(!chapter_files.is_empty(), "No chapter MP3 files found");

    // Now chapter_files[0] is the first chapter (1_*)
    let first_chapter = &chapter_files[0].path();
    println!("First chapter file: {}", first_chapter.display());

    // Example: verify first chapter metadata
    // Replace with actual expected titles and tracks from your test sample
    check_mp3_metadata(
        &chapter_files[0].path().to_string_lossy(),
        "Rick Astley // Never Gonna Give You Up",
        1,
    )?;

    Ok(())
}

/// Helper to check FLAC metadata for a given file
fn check_flac_metadata(flac_path: &str, expected_title: &str, expected_track: u32) -> Result<()> {
    // Use ffprobe to get JSON metadata
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format_tags=title,track",
            "-print_format",
            "json",
            flac_path,
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("ffprobe failed for '{}'", flac_path);
    }

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let tags = &json["format"]["tags"];

    let title = tags["title"].as_str().unwrap_or("");
    let track = tags["track"]
        .as_str()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    assert_eq!(title, expected_title, "Title mismatch for '{}'", flac_path);
    assert_eq!(track, expected_track, "Track mismatch for '{}'", flac_path);

    Ok(())
}

#[test]
fn test_m4b_extractor_flac_conversion() -> Result<()> {
    let binary_path = if cfg!(debug_assertions) {
        "./target/debug/m4b-extractor"
    } else {
        "./target/release/m4b-extractor"
    };

    let input_file = "tests/data/sample.m4b";
    let output_dir = "tests/output_flac";

    // Clean previous test output
    if Path::new(output_dir).exists() {
        fs::remove_dir_all(output_dir)?;
    }

    // Run the binary with FLAC conversion format
    let output = Command::new(binary_path)
        .args(&[
            input_file,
            "--output",
            output_dir,
            "--conversion-format",
            "flac",
        ])
        .output()?;

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success(), "Binary did not exit successfully");

    // Check that FLAC chapter files exist
    let mut chapter_files: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "flac")
                .unwrap_or(false)
        })
        .collect();

    // Sort by filename
    chapter_files.sort_by_key(|e| e.path().file_name().map(|f| f.to_os_string()));

    assert!(
        !chapter_files.is_empty(),
        "No chapter FLAC files found in {}",
        output_dir
    );

    // Verify first chapter metadata
    check_flac_metadata(
        &chapter_files[0].path().to_string_lossy(),
        "Rick Astley // Never Gonna Give You Up",
        1,
    )?;

    // Verify that no M4B files remain after conversion
    let m4b_files: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "m4b")
                .unwrap_or(false)
        })
        .collect();

    // Note: M4B files should still exist; conversion doesn't delete them by default
    // This verifies that the conversion process preserves original M4B files
    assert_eq!(m4b_files.len(), chapter_files.len(), "Mismatch between M4B and FLAC file counts");

    // Clean up after test
    fs::remove_dir_all(output_dir)?;

    Ok(())
}
