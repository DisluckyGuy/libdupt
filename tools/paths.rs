use std::{env, fs, io::{self, Write}, process};

pub fn list_path() -> String {
    String::from(format!("{}/sources/list.config", get_root_path()))
}

pub fn installed_path() -> String {
    String::from(format!("{}/sources/installed.config", get_root_path()))
}

pub fn get_root_path() -> String {
    let config =
        std::fs::read_to_string(format!("/home/mostafa/.dupt/configs/configs.conf")).unwrap();
    let mut root_path = String::new();
    for i in config.lines() {
        let key = i.split_once(":").expect("unable to split").0;
        let value = i.split_once(":").expect("unable to split").1;
        if key == "root_path" {
            root_path = String::from(value.trim());
        }
    }
    root_path
}

pub fn check_root_path() {
    if fs::File::open(format!("{}/.dupt/configs/configs.conf", get_root_path())).is_ok() {
        return;
    }
    let config_file =
        fs::File::create(format!("{}/.dupt/configs/configs.conf", get_root_path())).unwrap();
    let mut writer = io::BufWriter::new(config_file);
    let _user =
        String::from_utf8(process::Command::new("whoami").output().unwrap().stdout).unwrap();
    let user_name = _user.trim();
    writer
        .write(format!("root_path: /home/{}\n", user_name).as_bytes())
        .unwrap();
}
