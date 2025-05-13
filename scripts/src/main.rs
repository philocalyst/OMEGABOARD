#![recursion_limit = "512"]

use anyhow::Result;
use html::media::Audio;
use html::text_content::UnorderedList;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Where the sounds reside
    let sounds_dir = Path::new("/Users/philocalyst/Downloads/OSS-Soundboard/sounds");

    let mut all_files: Vec<String> = fs::read_dir(sounds_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().to_string_lossy().into_owned().into())
        .collect();
    all_files.sort_unstable();

    // Building only the first 100 for performance purposes, these are what comes with the first browser request.
    const MAX_INLINE: usize = 100;
    let mut ul = UnorderedList::builder();
    for name in all_files.iter().take(MAX_INLINE) {
        let src = format!("sounds/{}", name);
        let audio_element = Audio::builder()
            .data("src", src.clone())
            .controls("")
            .preload("")
            .build();
        ul.id("soundboard").list_item(|li| li.push(audio_element));
    }

    // Write out the template for a proper index.html
    let html_tree = ul.build();
    let page = format!(
        r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Soundboard</title>
  </head>
  <body>
    {}
    <!-- your lazy‐load script here -->
  </body>
</html>
"#,
        html_tree.to_string() // Feed in the rendered compoonent
    );
    fs::write("../index.html", page)?;

    // Add in the relative sounds path
    let all_files: Vec<String> = all_files
        .into_iter()
        .map(|path| format!("Sounds/{}", path))
        .collect();

    // Serialize and write
    let manifest = serde_json::to_string_pretty(&all_files)?;
    fs::write("../audioFiles.json", manifest)?;

    println!(
        "Wrote index.html (100 items) and audioFiles.json ({} items)",
        all_files.len()
    );
    Ok(())
}
