#[allow(dead_code)]

use anyhow::{Context, Result};

use clap::Parser;


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
    
    grrs::find_matches(&content, &args.pattern, std::io::stdout()).with_context(|| "error finding matches")?;

    Ok(())
}





#[test]
fn find_a_match() {
    let mut writer = Vec::new();
    let result = grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut writer).with_context(|| "error writing matches");
    match result {
       
        Ok(_) =>  assert_eq!(writer, b"lorem ipsum\n"),
        Err(err) => assert_eq!("error writing matches", err.to_string()),
    }
    

}