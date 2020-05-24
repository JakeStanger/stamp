use crate::cli::Command;
use ansi_term::{Color, Style};

impl Command {
    /// Prints a list of found templates.
    /// Shows their path when verbose mode is enabled.
    pub fn list(verbose: bool) -> Result<(), String> {
        let templates = Command::get_templates();

        println!(
            "{}\n",
            Style::new().bold().underline().paint("Installed templates")
        );

        for (template, path) in templates {
            println!(
                "• {} {}",
                template,
                if verbose {
                    Color::Green.paint(path.display().to_string())
                } else {
                    Color::Green.paint("")
                }
            );
        }

        Ok(())
    }
}
