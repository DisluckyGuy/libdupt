use std::{fs::{self, File}, io::Write, process::exit};

use crate::tools::{packages::{self, get_file}, paths::{self, get_root_path}, system};

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

        // for i in repo_dir {
        //     fs::remove_file(i.unwrap().path())?;
        // }
        for i in repositories.keys() {
            get_file(&"list.conf".to_string(), &format!("{}.json", i), i.as_str(), format!("{}/.dupt/sources/repositories", get_root_path()))?;
        }
        
        let mut easy = curl::easy::Easy::new();
        easy.url("https://raw.githubusercontent.com/DisluckyGuy/libdupt/main/dependencies.json").expect("failed to fetch");
        easy.write_function(|data|{
            let mut dep_file = File::create(format!("{}/.dupt/sources/dependencies.json", paths::get_root_path())).expect("failed to read dependencies");
            dep_file.write(data).expect("failed to write to dependency file");
            Ok(data.len())
        }).expect("failed to write");

        easy.perform().expect("failed to perform");

        


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
