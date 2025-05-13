#![recursion_limit = "512"]

use anyhow::Result;
use html::content::*;
use html::media::Audio;
use html::text_content::*;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Create the <ul> builder
    let mut ul = UnorderedList::builder();

    let mut count = 0;
    for entry in fs::read_dir("/Users/philocalyst/Downloads/OSS-Soundboard/sounds")? {
        let entry = entry?;
        // Owned Strings so we can move them into the closures
        let name = entry.file_name().to_string_lossy().into_owned();
        let src = entry.path().to_string_lossy().into_owned();

        // While testing keeping 100
        if count > 100 {
            break;
        }

        let audio_element = Audio::builder()
            .src(src.clone())
            .controls("") // Initialize controls
            .preload("") // No preload
            .build();

        ul.list_item(|li| li.push(audio_element));

        count += 1;
    }

    // Finish and render
    let tree = ul.build();
    println!("{}", tree.to_string());

    Ok(())
}
