use std::{
    fs::{self, File},
    path::Path,
};

use inquire::{autocompletion::Replacement, Autocomplete, CustomUserError};
use sonic_rs::{Deserialize, Serialize};
use toml;

const CONFIG_FILENAME: &str = "zippy.toml";

#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
    scopes: Option<Vec<String>>,
}

impl Settings {
    pub fn new() -> std::io::Result<Settings> {
        let file = Self::get_config_file()?;
        let parsed = toml::from_str::<Self>(&file).unwrap();
        Ok(parsed)
    }

    fn get_config_file() -> std::io::Result<String> {
        if !Path::new(CONFIG_FILENAME).exists() {
            Ok(String::new())
        } else {
            fs::read_to_string(CONFIG_FILENAME)
        }
    }

    pub fn scope_autocomplete(&self) -> ScopeAutocomplete {
        ScopeAutocomplete {
            scopes: self.scopes.clone().unwrap_or_default(),
        }
    }

    pub fn exists_scope(&mut self, scope: &str) -> bool {
        let Some(scopes) = &self.scopes else {
            return false;
        };
        scopes
            .iter()
            .any(|s| s.to_lowercase().trim() == scope.to_lowercase().trim())
    }

    pub fn add_scope(&mut self, scope: &str) {
        if !Path::new(CONFIG_FILENAME).exists() {
            File::create(CONFIG_FILENAME).unwrap();
        }

        let mut scopes = self.scopes.clone().unwrap_or_default();

        scopes.push(scope.to_owned());

        self.scopes = Some(scopes);

        let settings = toml::to_string_pretty(&self).unwrap();

        fs::write(CONFIG_FILENAME, settings).unwrap();
    }

    pub fn scopes(&self) -> Option<&Vec<String>> {
        self.scopes.as_ref()
    }
}

#[derive(Clone)]
pub struct ScopeAutocomplete {
    scopes: Vec<String>,
}

impl ScopeAutocomplete {
    fn filter_scopes(&mut self, input: &str) -> Vec<String> {
        self.scopes
            .iter()
            .filter(|s| {
                s.to_lowercase()
                    .trim()
                    .contains(input.to_lowercase().trim())
            })
            .cloned()
            .collect::<Vec<_>>()
    }
}

impl Autocomplete for ScopeAutocomplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        Ok(self.filter_scopes(input))
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.filter_scopes(input);

        Ok(match highlighted_suggestion {
            Some(sug) => Replacement::Some(sug),
            None => Replacement::None,
        })
    }
}

pub fn init() {
    // check_for_gitignore_file();
    // if the gitignore file doesn't exist, just return an error.
}
