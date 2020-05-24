use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use dirs::config_dir;

use crate::cli::Command;

pub mod list;
pub mod run;

impl Command {
    /// Reads the list of existing templates
    /// relative to the user's current path.
    ///
    /// This walks upwards, checking if each folder contains a `.stamp/templates`,
    /// before finally checking the `stamp/templates` folder in the user's config dir.
    fn read_templates(path: &Path, templates: &mut HashMap<String, PathBuf>, global: bool) {
        if path.is_dir() {
            let check_path =
                path.join(format!("{}stamp/templates", if !global { "." } else { "" }));

            if check_path.exists() && check_path.is_dir() {
                let extra_templates = read_dir(check_path)
                    .unwrap()
                    .filter(|entry| entry.as_ref().unwrap().path().is_dir())
                    .map(|entry| entry.unwrap().path())
                    .collect::<Vec<_>>();

                for template in extra_templates {
                    let template_name = template.file_name().unwrap().to_str().unwrap().to_string();

                    if !templates.contains_key(&template_name) {
                        templates.insert(template_name, template);
                    }
                }
            }

            if !global {
                let parent = path.parent();
                if parent.is_some() {
                    Command::read_templates(&parent.unwrap(), templates, global);
                }
            }
        }
    }

    /// Loads path-relative and global templates
    /// into a hashmap.
    pub fn get_templates() -> HashMap<String, PathBuf> {
        let current_path = current_dir().unwrap();
        let templates_path = config_dir().unwrap();

        let mut templates: HashMap<String, PathBuf> = HashMap::new();

        Command::read_templates(&current_path, &mut templates, false);
        Command::read_templates(&templates_path, &mut templates, true);

        templates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_found() {
        let templates = Command::get_templates();
        assert!(templates.contains_key("test"));
    }
}
