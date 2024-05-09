use std::{
    collections, error::Error, fs::{self, File}, io::BufReader, process, cmp
};

use super::{
    system,
    paths::{self, get_root_path},
};
use crate::package_data::PackageData;

pub fn search_package(name: &str) -> Result<PackageData, Box<dyn Error>> {
    println!("checking dir");
    let path = paths::get_root_path();
    let repo_dir = fs::read_dir(format!("{}/.dupt/sources/repositories", path))?;
    for i in repo_dir {
        let entry = i.unwrap();
        let file = fs::File::open(entry.path())?;
        let reader = BufReader::new(file);
        let info: Vec<PackageData> = serde_json::from_reader(reader)?;
        for j in info {
            if j.package_name == name {
                return Ok(j);
            }
        }
    }

    Err("Package not found")?
}

pub fn search_installed(name: &str) -> Result<PackageData, Box<dyn Error>> {
    let entries = fs::read_dir(&format!("{}/.dupt/installed", paths::get_root_path()))?;
    for i in entries {
        let entry = i.unwrap();
            println!("{}", entry.file_name().to_str().unwrap());
        if entry.file_name() == name.trim() {
            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);
            let package_data: PackageData = serde_json::from_reader(reader)?;
            return Ok(package_data);
        }
    }

    Err("Package not found")?
}

pub fn get_upgradable_packages() -> Vec<PackageData> {
    let mut packages: Vec<PackageData> = Vec::new();
    let current_pkgs = get_installed_packages();
    let mut repo_pkgs: Vec<PackageData> = Vec::new();
    let all_pkgs = get_packages();
    for i in 0..current_pkgs.len(){

        for j in  0..all_pkgs.len() {
            if current_pkgs[i].package_name == all_pkgs[j].package_name {
                repo_pkgs.push(all_pkgs[j].clone());
            }
        }

    }
    for i in 0..repo_pkgs.len() {
        let current_version: Vec<&str> = current_pkgs[i].version.split(".").collect();
        let repo_version: Vec<&str> = repo_pkgs[i].version.split(".").collect();


        if repo_version == current_version {
            continue;
        }

        for j in 0..cmp::min(repo_version.len(), current_version.len()){
            let current_number: i32 = current_version[j].parse().expect("failed to parse");
            let repo_number: i32 = repo_version[j].parse().expect("failed to parse");
            if current_number < repo_number {
                packages.push(repo_pkgs[i].clone());
                break;
            }
        }

        if repo_version.len() > current_version.len() {
            let split_version = repo_version.split_at(current_version.len()).0.to_vec();

            if split_version == current_version {
                packages.push(repo_pkgs[i].clone());
                break;
            }

        }

    }

    packages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_upgradable_packages(){
        let mut packages: Vec<PackageData> = Vec::new();
        let current_pkgs = get_installed_packages();
        let mut repo_pkgs: Vec<PackageData> = Vec::new();
        let all_pkgs = get_packages();
        for i in 0..current_pkgs.len(){

            for j in  0..all_pkgs.len() {
                println!("{}, {}", current_pkgs[i].package_name, all_pkgs[j].package_name);
                if current_pkgs[i].package_name == all_pkgs[j].package_name {
                    repo_pkgs.push(all_pkgs[j].clone());
                }
            }

        }
        println!("{}", repo_pkgs.len());
        for i in 0..repo_pkgs.len() {
            let current_version: Vec<&str> = current_pkgs[i].version.split(".").collect();
            let repo_version: Vec<&str> = repo_pkgs[i].version.split(".").collect();

            println!("{:?}, {:?}", current_version, repo_version);

            if repo_version == current_version {
                continue;
            }

            for j in 0..cmp::min(repo_version.len(), current_version.len()){
                let current_number: i32 = current_version[j].parse().expect("failed to parse");
                let repo_number: i32 = repo_version[j].parse().expect("failed to parse");
                if current_number < repo_number {
                    packages.push(repo_pkgs[i].clone());
                    break;
                }
            }

            if repo_version.len() > current_version.len() {
                let split_version = repo_version.split_at(current_version.len()).0.to_vec();

                println!("{:?}", split_version);

                if split_version == current_version {
                    packages.push(repo_pkgs[i].clone());
                    break;
                }

            }

        }

        println!("{:#?}", packages)
    }
}



pub fn get_dependency_count() -> collections::HashMap<String, i32> {
    let mut dependency_list: collections::HashMap<String, i32> = collections::HashMap::new();
    let installed_dir =
        fs::read_dir(&format!("{}/.dupt/installed", paths::get_root_path())).unwrap();
    for i in installed_dir {
        let entry = fs::read_to_string(i.unwrap().path()).unwrap();
        let info: PackageData = serde_json::from_str(&entry).expect("failed to read JSON string");
        for i in &info.make_dependencies {
            if dependency_list.contains_key(i) {
                *dependency_list.get_mut(i).unwrap() += 1;
            }
        }
    }
    dependency_list
}

pub fn clear_archives(name: &String) -> Result<(), Box<dyn Error>> {
    system::run_system_command(
        &format!(
            "rm {0}/.dupt/archives/{1}.tar.gz {0}/.dupt/archives/{1} -r",
            paths::get_root_path(),
            name
        ),
        true,
    )?;
    Ok(())
}

pub fn get_unused_dependencies(name: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut unused_dependencies: Vec<String> = Vec::new();
    let list = get_dependency_count();
    let pkg_dep = search_package(name)?.dependencies;

    for i in list.keys() {
        for j in &pkg_dep {
            if j != i {
                continue;
            }
            if list[i] > 1 {
                continue;
            }
            unused_dependencies.push(i.trim().into());
        }
    }

    Ok(unused_dependencies)
}

pub fn get_file(
    name: &String,
    output: &String,
    repo: &str,
    path: String,
) -> Result<(), Box<dyn Error>> {
    let mut repo_link = &String::new();
    let repositries = get_repos();
    for i in repositries.keys() {
        if i == &repo {
            repo_link = repositries.get(i).unwrap();
        }
    }
    println!("running curl");
    let pkg_loc = repo_link.split_once("||").unwrap();
    let _curl = process::Command::new("curl")
        .current_dir(path)
        .arg("-o")
        .arg(format!("{}", output))
        .arg(format!("{}{}{}", pkg_loc.0, name, pkg_loc.1))
        .arg("-l")
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn get_repos() -> collections::HashMap<String, String> {
    let mut repos: collections::HashMap<String, String> = collections::HashMap::new();
    let source_file =
        fs::read_to_string(format!("{}/.dupt/sources/sources.conf", get_root_path())).unwrap();
    for i in source_file.lines() {
        if i.trim().is_empty() {
            continue;
        }
        let line = i.split_once(":").unwrap();
        let name = line.0.trim().to_string();
        let link = line.1.trim().to_string();
        repos.insert(name, link);
    }
    println!("{:?}", repos);
    repos
}

pub fn get_packages() -> Vec<PackageData> {
    let mut packages: Vec<PackageData> = Vec::new();
    let list_dir = fs::read_dir(format!("{}/.dupt/sources/repositories", paths::get_root_path())).expect("couldn't read repositories directory");
    for i in list_dir {
        let entry = i.unwrap();
        let file = File::open(entry.path()).expect("failed to open file");
        let reader = BufReader::new(&file);
        let repo_packages: Vec<PackageData> = serde_json::from_reader(reader).expect("failed to read JSON");
        packages.extend(repo_packages);
    }
    packages
}

pub fn get_installed_packages() -> Vec<PackageData> {
    let mut packages: Vec<PackageData> = Vec::new();
    let installed_dir = fs::read_dir(format!("{}/.dupt/installed", paths::get_root_path())).expect("couldn't read repositories directory");
    for i in installed_dir {
        let entry = i.unwrap();
        let file = File::open(entry.path()).expect("failed to open file");
        let reader = BufReader::new(&file);
        let package: PackageData = serde_json::from_reader(reader).expect("failed to read JSON");
        packages.push(package);
    }
    packages
}
