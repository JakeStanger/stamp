use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, io};

use ansi_term::{Color, Style};
use handlebars::template::TemplateElement;
use handlebars::{Handlebars, Template};
use handlebars_misc_helpers::setup_handlebars;

use crate::cli::Command;

impl Command {
    /// Converts the CLI context vector into a hashmap.
    fn create_context_map(context: &Option<Vec<(String, String)>>) -> HashMap<String, String> {
        let mut context_map = HashMap::new();

        if context.is_some() {
            for param in context.as_ref().unwrap() {
                context_map.insert(param.0.clone(), param.1.clone());
            }
        }

        context_map
    }

    /// Extracts the parameter names from a template.
    fn extract_parameters(template: &Template) -> Vec<String> {
        template
            .elements
            .iter()
            .filter_map(|element| match element {
                TemplateElement::Expression(he) => {
                    if he.params.is_empty() {
                        he.name.as_name().map(String::from)
                    } else {
                        he.params.first().unwrap().as_name().map(String::from)
                    }
                }
                _ => None,
            })
            .collect()
    }

    /// Requests user input for each of the missing parameters
    /// and stores them in the context map.
    fn collect_missing_parameters(
        missing_parameters: Vec<String>,
        context: &mut HashMap<String, String>,
    ) {
        let num_missing = missing_parameters.len();

        println!(
            "{}",
            Style::new().bold().underline().paint("Missing Parameters")
        );

        println!("Some parameters were not specified and need to be collected.\n");

        for (i, param) in missing_parameters.iter().enumerate() {
            print!(
                "{} {}: ",
                Color::Blue.paint(format!("[{}/{}]", i + 1, num_missing)),
                param
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            context.insert(param.clone(), input.trim().to_string());
        }

        println!();
    }

    /// Writes each of the template files to disk,
    /// creating the directory tree first if required.
    fn write_files(rendered: HashMap<String, String>, out_path: PathBuf) -> io::Result<()> {
        for (name, template) in rendered {
            let path = out_path.join(&name);
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, template.as_bytes())?;

            println!("{} {}", Color::Green.paint("✓"), &name);
        }

        Ok(())
    }

    /// Collects parameters for a template,
    /// renders each of the files,
    /// and writes the files to disk.
    fn run_template(
        template: &PathBuf,
        context: &mut HashMap<String, String>,
        out_path: PathBuf,
    ) -> io::Result<()> {
        let mut handlebars = Handlebars::new();
        setup_handlebars(&mut handlebars);
        handlebars
            .register_templates_directory("", template)
            .unwrap();

        let mut name_handlebars = Handlebars::new();
        setup_handlebars(&mut name_handlebars);

        for (name, _template) in handlebars.get_templates() {
            name_handlebars
                .register_template_string(name, name)
                .unwrap();
        }

        let mut rendered: HashMap<String, String> = HashMap::new();
        for (name, template) in handlebars.get_templates() {
            let name_template = name_handlebars.get_template(name).unwrap();

            let mut parameters = Command::extract_parameters(name_template);
            parameters.append(&mut Command::extract_parameters(template));
            parameters.dedup();

            let missing_parameters: Vec<String> = parameters
                .into_iter()
                .filter(|param| !context.contains_key(param))
                .collect();

            if missing_parameters.len() > 0 {
                Command::collect_missing_parameters(missing_parameters, context);
            }

            let display_name = name_handlebars.render(name, context).unwrap();

            rendered.insert(display_name, handlebars.render(name, context).unwrap());
        }

        Command::write_files(rendered, out_path)
    }

    /// Takes a template name
    /// and runs the template.
    pub fn run(
        template: &String,
        context: &Option<Vec<(String, String)>>,
        out: &Option<PathBuf>,
    ) -> Result<(), String> {
        let templates = Command::get_templates();

        let template = templates
            .get(template)
            .ok_or_else(|| format!("Template {} not found", Color::Blue.paint(template)))?;

        let mut context_map = Command::create_context_map(context);

        let out_path = out.as_ref().unwrap_or(&current_dir().unwrap()).clone();

        Command::run_template(template, &mut context_map, out_path)
            .or_else(|error| Err(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn get_handlebars(dir_path: &str) -> Handlebars {
        let mut handlebars = Handlebars::new();
        setup_handlebars(&mut handlebars);

        handlebars
            .register_templates_directory("", dir_path)
            .unwrap();

        handlebars
    }

    #[test]
    fn context_map_created() {
        let context: Option<Vec<(String, String)>> = Some(vec![
            ("greeting".to_string(), "Hello".to_string()),
            ("subject".to_string(), "world".to_string()),
        ]);

        let context_map = Command::create_context_map(&context);

        assert!(context_map.contains_key("greeting"));
        assert!(context_map.contains_key("subject"));

        assert_eq!(context_map.get("greeting").unwrap(), "Hello");
        assert_eq!(context_map.get("subject").unwrap(), "world");
    }

    #[test]
    fn template_parameters_extracted() {
        let handlebars = get_handlebars(".stamp/templates/test");
        let template = handlebars.get_template("{{greeting}}").unwrap();

        let parameters = Command::extract_parameters(template);
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0], "subject");
    }

    #[test]
    fn rendered_template_written() {
        let mut context: HashMap<String, String> = HashMap::new();

        context.insert("greeting".to_string(), "Hello".to_string());
        context.insert("subject".to_string(), "world".to_string());

        Command::run_template(
            &PathBuf::from(".stamp/templates/test"),
            &mut context,
            PathBuf::from("/tmp/stamp"),
        );

        assert!(PathBuf::from("/tmp/stamp/Hello").exists());
        assert_eq!(fs::read_to_string("/tmp/stamp/Hello").unwrap(), "world");
    }
}
