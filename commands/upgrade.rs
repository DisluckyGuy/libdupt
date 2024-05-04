use std::{fs, process::{exit, Command}};

use crate::tools::{self, packages::{self, get_file}, paths::get_root_path, system, terminal::{confirm, print_green}};

use super::{install::{self, Install}, remove::Remove};

struct Upgrade {
    confirm: bool,
}

impl Default for Upgrade {
    fn default() -> Self {
        Self { confirm: true }
    }
}

impl super::Command for Upgrade {
    
}
impl Upgrade {
    fn help(&self) {
        println!("upgrade packages");
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::make_dupt_folder()?;
        
        let upgradable = packages::get_upgradable_packages();

        if upgradable.len() == 0 {
            print_green("no packages to upgrade");
            return Ok(());
        }

        let mut args: Vec<String> = Vec::new();

        for package in upgradable {

            args.push(package.package_name);
        }

        println!("packages to upgrade: ");

        for i in &args {
            println!("{}", i);
        }
        println!();
        if self.confirm {
            confirm("do you want to continue[y/n]?: ")?;
            println!()
        }
        


        args.push("-y".to_string());
        let remove = Remove::from_args(&args)?;
        remove.run()?;
        let install = Install::from_args(&args)?;
        install.run()?;

        print_green("upgrade successful");
        Ok(())
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut command = Self::default();
        if args.is_empty() {
            return Ok(command);
        }
        if args.last().expect("no last argument") == "-y" {
            command.confirm = false;
        }
        if args.contains(&"help".to_string()) {
            command.help();
            exit(0);
        }
        Ok(command)
    }
}