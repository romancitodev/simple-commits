use crate::tui::structs::Scope;

use super::cli::{InitOptions, SimpleCommitsConfig};
use directories::BaseDirs;
use merge2::Merge;
use std::{env::current_dir, path::PathBuf};

#[inline]
pub fn swap_option<T>(left: &mut Option<T>, right: &mut Option<T>) {
    if left.is_none() || right.is_some() {
        core::mem::swap(left, right);
    }
}
pub fn create_config(option: InitOptions) -> PathBuf {
    match option {
        InitOptions::Global => {
            let path = BaseDirs::new().unwrap().config_dir().join("sc");
            let path = path.join("config.toml");
            if !path.exists() {
                std::fs::create_dir_all(path.clone()).unwrap();
            }
            path
        }
        InitOptions::Local => {
            let path = current_dir()
                .expect("The current dir must exists")
                .join("sc.toml");
            if !path.exists() {
                std::fs::create_dir_all(path.clone()).unwrap();
            }
            path
        }
    }
}

/// (Global, Local)
type ConfigPaths = (PathBuf, Option<PathBuf>);

pub fn load_config(path: Option<PathBuf>, config: &mut SimpleCommitsConfig) -> ConfigPaths {
    let global_path = path.unwrap_or(create_config(InitOptions::Global));

    if let Ok(content) = std::fs::read_to_string(&global_path) {
        let mut global_config: SimpleCommitsConfig = toml::from_str(&content).unwrap();
        config.config.clone_from(&global_path);
        config.scopes = Some(Scope::default());
        config.merge(&mut global_config);
    }

    let local_path = current_dir().map(|path| path.join("sc.toml"));

    if let Ok(local_path_ok) = &local_path {
        if let Ok(content) = std::fs::read_to_string(local_path_ok) {
            let mut local_config: SimpleCommitsConfig = toml::from_str(&content).unwrap();
            config.config.clone_from(local_path_ok);
            config.scopes = Some(Scope::default());
            config.merge(&mut local_config);
        }
    }

    (global_path, local_path.ok())
}
