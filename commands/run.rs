use std::process::{self, exit};

use crate::{package_data, tools::{packages::{self, search_installed}, paths::{self, get_root_path}, system}};

use super::Command;

pub struct Run {
    pub name: String,
}

impl Command for Run {

}

impl Default for Run {
    fn default() -> Self {
        Self {
            name: String::from("help"),
        }
    }
}

impl Run {
    fn help(&self) {
        println!("run");
        println!("run installed software");
    }


    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        system::make_dupt_folder()?;

        let package = packages::search_installed(&self.name)?;
        
        std::env::set_current_dir(&format!("{}/.dupt/bin/{}", get_root_path(), package.package_name))?;
        system::run_system_command("sh -c ./run.sh", true)?;
        Ok(())
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut command = Self::default();
        if args.len() == 0 {
            Err("not enought arguments")?
        }
        command.name = String::from(&args[0]);
        if command.name == "help".to_string() {
            command.help();
            exit(0);
        }
        Ok(command)
    }

}
