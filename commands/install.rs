use std::{
    env,
    fs,
    process::{exit},
};

use crate::tools::{
    packages::{self, search_package},
    paths,
    system::{self, get_package_manager},
    terminal,
};

use super::Command;
pub struct Install {
    pub names: Vec<String>,
    pub confirm: bool,
}

impl Command for Install {}

impl Default for Install {
    fn default() -> Self {
        Self {
            names: vec![String::from("help")],
            confirm: true,
        }
    }
}

impl Install {
    fn help(&self) {
        println!("install");
        println!();
        println!("install software from different repositories");
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        system::make_dupt_folder()?;

        for name in &self.names {
            if packages::search_installed(name).is_ok() {
                terminal::print_green("package already installed");
                return Ok(());
            }
        }

        terminal::print_blue("searching packages");

        for name in &self.names {
            search_package(name)?;
        }

        println!("packages found");

        if self.confirm {
            println!();
            println!("packages to install:");
            println!();

            for i in &self.names {
                println!("{}", i);
            }
            println!();

            let cont = terminal::confirm("Do you want to continue? [y/n]:")?;
            println!();
            if !cont {
                println!();
                println!("aborting...");
                return Ok(());
            }
        }

        terminal::print_blue("installing dependencies");

        let mut dependencies: Vec<String> = Vec::new();



        for name in &self.names {
            let mut pkginfo = search_package(&name)?;
            dependencies.append(&mut pkginfo.dependencies);
        }

        system::install_system_packages(dependencies, get_package_manager())?;

        for name in &self.names {
            env::set_current_dir(format!("{}/.dupt", paths::get_root_path()))?;

            terminal::print_blue("downloading package");

            packages::get_file(
                &format!("{}.tar.gz", name),
                &format!("{}.tar.gz", name),
                "dupt-repo-main",
                format!("{}/.dupt/archives", paths::get_root_path()),
            )?;
            println!("downloaded");

            let tar_file = fs::File::open(format!(
                "{}/.dupt/archives/{}.tar.gz",
                paths::get_root_path(),
                name
            ))?;

            let tar = flate2::read::GzDecoder::new(tar_file);
            let mut archive = tar::Archive::new(tar);
            println!("unpacking");
            archive.unpack(format!("{}/.dupt/archives", paths::get_root_path()))?;


            env::set_current_dir(format!(
                "{}/.dupt/archives/{}/control",
                paths::get_root_path(),
                name
            ))?;

            println!();
            println!("running preinstall configurations");

            system::run_system_command("sh ./preinst.sh", true)?;

            println!();
            terminal::print_blue("building..");

            system::run_system_command("sh ./build.sh", true)?;

            println!();
            println!("running post configurations");

            system::run_system_command("sh ./postinst.sh", true)?;

            system::run_system_command(&format!("cp {0}/.dupt/archives/{1}/PKGINFO.json {0}/.dupt/installed/{1}", paths::get_root_path(), name), true)?;

        }

        println!("cleaning archives");
        for name in &self.names {
            packages::clear_archives(&name)?;
        }

        println!();
        terminal::print_green("finished successfully");
        Ok(())
    }

    pub fn from_args(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut command = Install::default();

        if args.len() == 0 {
            command.help();
            return Err("Not enough arguments")?;
        }

        if args.last().unwrap() == "-y" {
            command.confirm = false;
        }

        if args.len() == 1 && command.confirm == false {
            command.help();
            return Err("Not enough arguments")?;
        }

        if !command.confirm {
            command.names = args[0..args.len() - 1].to_vec();
        } else {
            command.names = args.to_vec();
        }

        if command.names.contains(&"help".to_string()) {
            command.help();
            exit(0);
        }
        Ok(command)
    }
}
