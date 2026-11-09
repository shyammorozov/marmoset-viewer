//! Windows registry read helpers for Marmoset Viewer.

pub const APP_KEY: &str = "Software\\Marmoset\\Marmoset Viewer";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Marmoset Viewer";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Marmoset Viewer") },
        RegValue { name: "Version",     value: String::from("v4.0.2") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Marmoset Viewer") },
        RegValue { name: "Publisher",       value: String::from("Marmoset") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Marmoset Viewer\\uninstall.exe\"") },
    ]
}
