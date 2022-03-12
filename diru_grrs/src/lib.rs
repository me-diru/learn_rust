use anyhow::{Context, Result};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| "error writing mathces")?;
            // return Err(anyhow::Error::msg("error writing mathces"));
        }
    }
    Ok(())
}