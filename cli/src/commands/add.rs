use derive_more::Display;
use derive_more::Error;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use crate::command::Command;
use crate::command::CommandResult;
use clap::ArgMatches;
use log::*;
use toml_edit::{Document, value, table};

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "Could not find Smaug.toml at {}", "config_path.display()")]
    FileNotFound { config_path: PathBuf }/*,
    #[display(
        fmt = "Could not parse Smaug.toml at {}: {}",
        "path.display()",
        "parent"
    )]
    ParseError {
        path: PathBuf,
        parent: toml::de::Error,
    },*/
}

#[derive(Debug)]
pub struct Add;

impl Command for Add {
    fn run(&self, matches: &ArgMatches) -> CommandResult {
        trace!("Add Command");

        let current_directory = env::current_dir().unwrap();
        let directory: &str = matches
            .value_of("path")
            .unwrap_or_else(|| current_directory.to_str().unwrap());
        debug!("Directory: {}", directory);
        let canonical = std::fs::canonicalize(directory)?;
        let path = Path::new(&canonical);
        let path = std::fs::canonicalize(&path).expect("Could not find path");

        let config_path = path.join("Smaug.toml");

        if !config_path.is_file() {
            return Err(Box::new(Error::FileNotFound { config_path }));
            //Err(Error::FileNotFound { config_path });
            //Err(err) => Err(Box::new(config_path));
        }

        let config = std::fs::read_to_string(config_path.clone()).expect("Could not read Smaug.toml");

        //let config = smaug::config::load(&config_path)?;
        debug!("Smaug config: {:?}", config);

        let mut doc = config.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), config);

        //doc["dependencies"].as_table_mut().unwrap().push(value("1.0.0"));
        
        doc["dependencies"] = table();
        doc["dependencies"]["hello"] = value("2.2.2");

        debug!("Smaug config: {:?}", doc.to_string());


        //config["a"]["b"]["c"]["d"] = value("hello");

        


        Ok(Box::new("Stub add command is working!"))
    }
}
