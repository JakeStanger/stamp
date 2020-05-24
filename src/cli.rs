use std::error::Error;
use std::path::PathBuf;

use structopt::StructOpt;

/// Parse a single cli key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum Command {
    List {
        /// Shows the path of each template on disk
        #[structopt(short, long)]
        verbose: bool,
    },
    Run {
        /// The template name
        template: String,

        /// The path to create the template.
        /// If not specified, the CWD is used.
        #[structopt(short, long, parse(from_os_str))]
        out: Option<PathBuf>,

        /// A set of `key=value` pairs
        /// used to render the variables inside the template.
        #[structopt(short, long, parse(try_from_str = parse_key_val))]
        context: Option<Vec<(String, String)>>,
    },
}
