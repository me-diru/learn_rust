#[allow(dead_code)]

use anyhow::{Context, Result};

use clap::Parser;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("Couldn't read this file");
    
    find_matches(&content, &args.pattern, std::io::stdout()).with_context(|| "error finding matches")?;

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| "error writing mathces")?;
            // return Err(anyhow::Error::msg("error writing mathces"));
        }
    }
    Ok(())
}



#[test]
fn find_a_match() {
    let mut writer = Vec::new();
    let result = find_matches("ipsum\ndolor sit amet", "lorem", &mut writer).with_context(|| "error writing matches");
    match result {
       
        Ok(_) =>  assert_eq!(writer, b"lorem ipsum\n"),
        Err(err) => assert_eq!("error writing matches", err.to_string()),
    }
    

}