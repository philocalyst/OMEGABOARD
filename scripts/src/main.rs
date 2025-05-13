#![recursion_limit = "512"]
use anyhow::Result;
use html::media::Audio;
use html::text_content::UnorderedList;
use std::error::Error;
use std::fs;
fn main() -> Result<(), Box<dyn Error>> {
    let mut unordered_list = UnorderedList::builder();

    let audio_files = fs::read_dir("../../sounds")?;

    for audio_file in audio_files {
        audio_file.expect("Shouldn't every entry exist if the directory exists?");

        unordered_list.list_item(|li| li.text(audio_file));
    }
    let string = tree.to_string();
    println!("{string}");
    Ok(())
}
