use std::process;

pub fn list_path() -> String {
    String::from(format!("{}/sources/list.config", get_root_path()))
}

pub fn installed_path() -> String {
    String::from(format!("{}/sources/installed.config", get_root_path()))
}

pub fn get_root_path() -> String {
    let whoami = process::Command::new("whoami").output().expect("failed to run whoami");
    let user = String::from_utf8(whoami.stdout.to_vec()).expect("failed to turn utf8 to string");
    let root_path = "/home/".to_string() + &user.trim();
    root_path.trim().to_string()
}
