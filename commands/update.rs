use std::{fs, process::exit};

use crate::tools::{system, packages::{self, get_file}, paths::get_root_path};

use super::Command;

pub struct Update {
}
 impl Command for Update {

 }

 impl Default for Update {
    fn default() -> Self {
        return Update {};
    }
 }

impl Update {
    fn help(&self) {
        println!("update package list");
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::make_dupt_folder()?;

        let repositories = packages::get_repos();
        let repo_dir = fs::read_dir(format!("{}/.dupt/sources/repositories", get_root_path()))?;

        // for i in repo_dir {
        //     fs::remove_file(i.unwrap().path())?;
        // }
        for i in repositories.keys() {
            get_file(&"list.conf".to_string(), &format!("{}.json", i), i.as_str(), format!("{}/.dupt/sources/repositories", get_root_path()))?;
        }
        
        let easy = curl::easy::Easy::new();


        Ok(())
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let command = Self::default();
        if args.is_empty() {
            return Ok(command);
        }
        if args.contains(&"help".to_string()) {
            command.help();
            exit(0);
        }
        Ok(command)
    }
}
