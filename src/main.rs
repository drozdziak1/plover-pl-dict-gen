use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

static DICT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/odm.txt"));

static OUT_FILE_PATH: &'static str = "out.txt";

pub type ErrBox = Box<dyn std::error::Error>;

fn main() -> Result<(), ErrBox> {
    println!("Starting...");

    let mut word_set: BTreeSet<String> = BTreeSet::new();

    let mut file_line_count = 0u32;
    let mut file_word_count = 0u64;
    let mut file_duplicate_count = 0u32;

    let mut file_non_alphanum_count = 0u32;

    for line in DICT.lines() {
        for word in line.split(", ") {
            let sanitized = word.trim().to_lowercase();

            if sanitized
                .chars()
                .any(|ch| !ch.is_alphanumeric() || ch.is_whitespace())
            {
                file_non_alphanum_count += 1;
            } else {
                if word_set.contains(&sanitized) {
                    file_duplicate_count += 1;
                } else {
                    word_set.insert(sanitized);
                }
            }

            file_word_count += 1;
        }
        file_line_count += 1;
    }

    println!(
        "Processed {} words in {} lines, discarded {} duplicates and {} non-alphanumerical and/or multi-word entries",
        file_word_count, file_line_count, file_duplicate_count, file_non_alphanum_count
    );

    let out_buf: String = word_set.into_iter().collect::<Vec<_>>().join("\n");

    let mut out_f = File::create(OUT_FILE_PATH)?;

    out_f.write_all(out_buf.as_bytes())?;

    println!("Written out to {}", OUT_FILE_PATH);

    Ok(())
}
