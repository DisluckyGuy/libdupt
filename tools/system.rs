use std::{env, error::Error, process::{self, exit}};



use crate::tools::paths;

pub enum PackageManager {
    Dnf,
    Pacman,
    Apt,
    Zypper,
    Apk
}
pub fn make_dupt_folder() -> Result<(), Box<dyn Error>> {

    if env::consts::OS == "linux" {
        println!("chowning");
        let _chown = process::Command::new("chmod")
            .arg("+x")
            .arg(format!("{}/.dupt/scripts/*", paths::get_root_path()));
    }

    run_system_command(
        &format!(
            "sh {}/.dupt/scripts/mkdupt.sh {}",
            paths::get_root_path(),
            paths::get_root_path()
        ),
        false,
    )?;
    Ok(())
}

pub fn get_package_manager() -> PackageManager {

    let exec_path = std::env::current_exe().unwrap().to_str().unwrap().to_string();
    println!("{}", exec_path);

    if exec_path.contains("/app") {

        let pacman = process::Command::new("flatpak-spawn").arg("--host").arg("pacman").arg("-V").output();
        let dnf = process::Command::new("flatpak-spawn").arg("--host").arg("dnf").arg("--version").output();
        let apt = process::Command::new("flatpak-spawn").arg("--host").arg("apt").arg("-v").output();
        let apk = process::Command::new("flatpak-spawn").arg("--host").arg("apk").arg("--version").output();
        let zypper = process::Command::new("flatpak-spawn").arg("--host").arg("zypper").arg("-V").output();

        if pacman.is_ok() && pacman.unwrap().status.success() {
            return PackageManager::Pacman;
        } else if dnf.is_ok() && dnf.unwrap().status.success() {
            println!("{}", "fetch successful");
            return PackageManager::Dnf;


        } else if apt.is_ok() && apt.unwrap().status.success() {
            return PackageManager::Apt;
        } else if apk.is_ok() && apk.unwrap().status.success() {
            return PackageManager::Apk;
        } else if zypper.is_ok() && zypper.unwrap().status.success() {
            return PackageManager::Zypper;
        } else {
            println!("All package manager commands failed, make sure you have one of these package managers");
            exit(1);
        }
    }
    let pacman = process::Command::new("pacman").arg("-V").output();
    let dnf = process::Command::new("dnf").arg("--version").output();
    let apt = process::Command::new("apt").arg("-v").output();
    let apk = process::Command::new("apk").arg("--version").output();
    let zypper = process::Command::new("zypper").arg("-V").output();
    if pacman.is_ok() && pacman.unwrap().status.success() {
        return PackageManager::Pacman;
    } else if dnf.is_ok() && dnf.unwrap().status.success() {
        return PackageManager::Dnf;
    } else if apt.is_ok() && apt.unwrap().status.success() {
        return PackageManager::Apt;
    } else if apk.is_ok() && apk.unwrap().status.success() {
        return PackageManager::Apk;
    } else if zypper.is_ok() && zypper.unwrap().status.success() {
        return PackageManager::Zypper;
    } else {
        println!("All package manager commands failed, make sure you have one of these package managers");
        exit(1);
    }
}

pub fn install_system_packages(packages: Vec<String>, manager: PackageManager) -> Result<(), Box<dyn Error>>{
    let exec_path = std::env::current_exe().unwrap().to_str().unwrap().to_string();

    if exec_path.contains("/app") {
        match manager {
            PackageManager::Dnf => {
                let _command = process::Command::new("flatpak-spawn").arg("--host").arg("pkexec").arg("dnf").arg("install").args(packages).arg("-y").spawn()?.wait()?;
                return Ok(());
            },
            PackageManager::Pacman => {
                let _command = process::Command::new("flatpak-spawn").arg("--host").arg("pkexec").arg("pacman").arg("-S").args(packages).arg("-y").spawn()?.wait()?;
                return Ok(());
            },
            PackageManager::Apt => {
                let _command = process::Command::new("flatpak-spawn").arg("--host").arg("pkexec").arg("apt").arg("install").args(packages).arg("-y").spawn()?.wait()?;
                return Ok(());
            },
            PackageManager::Zypper => {
                let _command = process::Command::new("flatpak-spawn").arg("--host").arg("pkexec").arg("Zypper").arg("install").args(packages).arg("-y").spawn()?.wait()?;
            return Ok(());
            },
            PackageManager::Apk => {
                let _command = process::Command::new("flatpak-spawn").arg("--host").arg("pkexec").arg("Apk").arg("install").args(packages).arg("-y").spawn()?.wait()?;
                return Ok(());
        },
    }
    }
    match manager {
        PackageManager::Dnf => {
            let _command = process::Command::new("sudo").arg("dnf").arg("install").args(packages).arg("-y").spawn()?.wait()?;
            return Ok(());
        },
        PackageManager::Pacman => {
            let _command = process::Command::new("sudo").arg("pacman").arg("-S").args(packages).arg("-y").spawn()?.wait()?;
            return Ok(());
        },
        PackageManager::Apt => {
            let _command = process::Command::new("sudo").arg("apt").arg("install").args(packages).arg("-y").spawn()?.wait()?;
            return Ok(());
        },
        PackageManager::Zypper => {
            let _command = process::Command::new("sudo").arg("Zypper").arg("install").args(packages).arg("-y").spawn()?.wait()?;
            return Ok(());
        },
        PackageManager::Apk => {
            let _command = process::Command::new("sudo").arg("Apk").arg("install").args(packages).arg("-y").spawn()?.wait()?;
            return Ok(());
        },
    }
}

pub fn run_system_command(command: &str, spawn: bool) -> Result<(), Box<dyn Error>>{
    let args: Vec<&str> = command.split_whitespace().collect();
    let binding = std::env::current_exe()?;
    let exec_path = binding.to_str().unwrap();
    if exec_path.contains("/app") {
        let mut _command = std::process::Command::new("flatpak-spawn");
        _command.arg("--host").args(args);
        if spawn {
            _command.spawn()?.wait()?;
        }
    } else {
        if args.len() == 1 {
            let mut _command = std::process::Command::new(args[0]);
            if spawn {
                _command.spawn()?.wait()?;
            }
            return Ok(())
        }

        let mut _command = std::process::Command::new(args[0]);
        _command.args(args[1..args.len()].to_vec());
        if spawn {
            let _child = _command.spawn()?.wait()?;
        }
        return Ok(())
    }

    Ok(())
}
