use crate::utils::{needs_sanitize, sanitize_filename};
use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use serde_json::Value;
use std::fs;
use std::{path::Path, process::Command};

use crate::args::Args;

pub fn split_chapters(args: &Args, output_dir: &str, json: &Value) -> Result<()> {
    let chapters = json["chapters"]
        .as_array()
        .context("No chapters found in metadata")?;

    for (idx, chapter) in chapters.iter().enumerate() {
        let start = chapter["start_time"]
            .as_str()
            .context("Chapter missing start_time")?;
        let end = chapter["end_time"]
            .as_str()
            .context("Chapter missing end_time")?;

        // Original title (for metadata)
        let original_title = chapter["tags"]["title"]
            .as_str()
            .unwrap_or("Chapter")
            .to_string();

        // Sanitized filename
        let mut safe_title = original_title.clone();

        if args.sanitize {
            safe_title = sanitize_filename(&safe_title);
        } else if needs_sanitize(&safe_title) {
            let sanitized = sanitize_filename(&safe_title);

            if sanitized != safe_title {
                eprintln!(
                    "⚠️ Invalid characters detected in chapter title. \
Sanitization applied automatically.\n\
→ Before: {}\n→ After : {}\n\
(Use --sanitize to always sanitize filenames.)",
                    safe_title, sanitized
                );
                safe_title = sanitized;
            }
        }

        let filename = format!("{}_{}.m4b", idx + 1, safe_title);
        let output_path = Path::new(output_dir).join(&filename);

        println!("🎵 Chapter {}: {}", idx + 1, original_title);

        let track_num = (idx + 1).to_string();

        let status = Command::new("ffmpeg")
            .args([
                "-loglevel",
                "error",
                "-y",
                "-i",
                &args.input,
                "-ss",
                start,
                "-to",
                end,
                "-c",
                "copy",
                // Metadata
                "-metadata",
                &format!("title={}", original_title),
                "-metadata",
                &format!("track={}", track_num),
                output_path.to_str().unwrap(),
            ])
            .status()
            .context("Failed to run ffmpeg for splitting chapters")?;

        if !status.success() {
            bail!("ffmpeg failed to split chapter {}", idx + 1);
        }
    }
    Ok(())
}

pub fn convert_to_mp3(output_dir: &str, quality: u8) -> Result<()> {
    println!("\n🔄 Converting all chapter .m4b files to .mp3 in parallel (fast encoding) and deleting originals...");

    let entries: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|e| {
            e.ok().and_then(|entry| {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("m4b") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    entries.par_iter().for_each(|m4bfile| {
        let mp3file = m4bfile.with_extension("mp3");
        println!(
            "Converting '{}' → '{}'",
            m4bfile.display(),
            mp3file.display()
        );

        let status = Command::new("ffmpeg")
            .args([
                "-loglevel",
                "error",
                "-y",
                "-i",
                m4bfile.to_str().unwrap(),
                // ❌ remove cover image
                "-map",
                "0:a",
                "-map",
                "-0:v",
                // Encoding
                "-acodec",
                "libmp3lame",
                "-qscale:a",
                &quality.to_string(),
                mp3file.to_str().unwrap(),
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                let _ = fs::remove_file(m4bfile);
            }
            Ok(_) => eprintln!("Conversion failed for '{}'", m4bfile.display()),
            Err(e) => eprintln!("Failed to run ffmpeg for '{}': {}", m4bfile.display(), e),
        }
    });

    println!("✅ Conversion and cleanup complete.");
    Ok(())
}
