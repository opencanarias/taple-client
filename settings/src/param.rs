use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::{collections::HashSet, hash::Hasher};

use clap::{Arg, ArgAction, ArgMatches, Command};
use toml::Value;

use crate::Error;
use crate::utils::check_if_valid_env;

#[derive(Hash, PartialEq, Eq)]
pub enum ParamType {
    Enum(Vec<String>),
    RequiredSet,
    Flag,
    Set,
}

pub struct SettingSchemaBuilder {
    id: String,
    long: Option<String>,
    short: Option<char>,
    help: Option<String>,
    param_type: Option<ParamType>,
    section: Option<String>,
    hidden: bool
}

impl SettingSchemaBuilder {
    pub fn new<T: Into<String>>(id: T) -> Result<Self, Error> {
        let id: String = id.into();
        if id.is_empty() {
            return Err(Error::EmptyString);
        }
        if !check_if_valid_env(&id) {
            return Err(Error::InvalidStringForEnv(id));
        }
        Ok(Self {
            id,
            long: None,
            short: None,
            help: None,
            param_type: None,
            section: None,
            hidden: false
        })
    }

    pub fn build(mut self) -> SettingSchema {
        let id = self.id;
        SettingSchema {
            id: id.clone(),
            short: self.short.take(),
            long: self.long.unwrap_or(id.clone()),
            env: id.clone().to_uppercase(),
            param_type: self.param_type.unwrap_or(ParamType::Set),
            help: self.help.unwrap_or(id),
            hidden: self.hidden,
            section: self.section,
        }
    }

    pub fn short(mut self, value: char) -> Self {
        self.short = Some(value);
        self
    }

    pub fn param_type(mut self, value: ParamType) -> Self {
        self.param_type = Some(value);
        self
    }

    pub fn long<T: Into<String>>(mut self, value: T) -> Self {
        self.long = Some(value.into());
        self
    }

    pub fn help<T: Into<String>>(mut self, value: T) -> Self {
        self.help = Some(value.into());
        self
    }

    pub fn section<T: Into<String>>(mut self, value: T) -> Self {
        self.section = Some(value.into());
        self
    }

    pub fn hide(mut self, value: bool) -> Self {
        self.hidden = value;
        self
    }
}

#[derive(Eq)]
pub struct SettingSchema {
    id: String,
    short: Option<char>,
    long: String,
    env: String,
    param_type: ParamType,
    help: String,
    hidden: bool,
    pub(crate) section: Option<String>,
}

impl PartialEq for SettingSchema {
    fn eq(&self, other: &Self) -> bool {
        (self.id == other.id)
            || (self.short == other.short)
            || (self.long == other.long)
            || (self.env == other.env)
    }
}

impl Hash for SettingSchema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl SettingSchema {
    pub fn to_arg(&self) -> Arg {
        let mut result = Arg::new(self.id.clone());
        if let Some(short) = self.short {
            result = result.short(short);
        };
        if let Some(section) = &self.section {
            result = result.help_heading(section);
        };
        let mut result = match &self.param_type {
            ParamType::Enum(data) => {
                result.value_parser(data.clone())
            }
            ParamType::RequiredSet => result.required(true).action(ArgAction::Set),
            ParamType::Flag => result.action(ArgAction::SetTrue),
            ParamType::Set => result.action(ArgAction::Set),
        };
        result.long(self.long.clone()).help(self.help.clone()).hide(self.hidden)
    }
}

pub struct ConfigGenerator {
    data: HashSet<SettingSchema>,
    toml_filename: Option<String>,
    program_name: Option<String>,
    author: Option<String>,
    about: Option<String>,
    usage: Option<String>,
    prefix: Option<String>,
}

impl ConfigGenerator {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
            toml_filename: None,
            program_name: None,
            author: None,
            about: None,
            usage: None,
            prefix: None,
        }
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }

    pub fn add_setting(mut self, settings: SettingSchema) -> Self {
        self.data.insert(settings);
        self
    }

    pub fn add_toml<T: Into<String>>(mut self, filename: T) -> Self {
        self.toml_filename = Some(filename.into());
        self
    }

    pub fn program_name<T: Into<String>>(mut self, program_name: T) -> Self {
        self.program_name = Some(program_name.into());
        self
    }

    pub fn author<T: Into<String>>(mut self, author: T) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn about<T: Into<String>>(mut self, about: T) -> Self {
        self.about = Some(about.into());
        self
    }

    pub fn group<T: Into<String>>(mut self, group: T, settings: Vec<SettingSchema>) -> Result<Self, Error> {
        let group: String = group.into();
        if !check_if_valid_env(&group) {
            return Err(Error::InvalidStringForEnv(group));
        }
        settings.into_iter().for_each(|mut s| {
            s.section = Some(group.clone());
            self.data.insert(s);
        });
        Ok(self)
    }

    pub fn prefix<T: Into<String>>(mut self, prefix: T) -> Result<Self, Error> {
        let prefix = prefix.into();
        if !check_if_valid_env(&prefix) {
            return Err(Error::InvalidStringForEnv(prefix));
        }
        self.prefix = Some(prefix.into());
        Ok(self)
    }

    fn get_matches(&mut self) -> ArgMatches {
        let program_name = self
            .program_name
            .take()
            .unwrap_or(env!("CARGO_PKG_NAME").into());
        let command = Command::new(program_name.clone());
        let command = if let Some(author) = self.author.take() {
            command.author(author)
        } else {
            command
        };
        let command = if let Some(about) = self.about.take() {
            command.about(about)
        } else {
            command
        };
        let mut command = command
            .version(env!("CARGO_PKG_VERSION"))
            .override_usage(self.usage.take().unwrap_or(program_name));
        for setting in self.data.iter() {
            command = command.arg(setting.to_arg());
        }
        command.get_matches()
    }

    fn get_toml(&mut self) -> Option<toml::Table> {
        let Some(filename) = &self.toml_filename else {
        return  None;
      };
        let Ok(file) = fs::read_to_string(filename) else {
        return  None;
      };
        Some(file.parse::<toml::Table>().unwrap())
    }

    fn get_from_matches(setting: &SettingSchema, matches: &ArgMatches) -> Option<String> {
        if let Some(_) = matches.value_source(&setting.id) {
            if let ParamType::Flag = &setting.param_type {
                Some(matches.get_one::<bool>(&setting.id).unwrap().to_string())
            } else {
                matches.get_one::<String>(&setting.id).cloned()
            }
        } else {
            None
        }
    }

    fn get_string_from_value(value: &Value) -> Option<String> {
        match value {
            Value::Boolean(data) => Some(data.to_string()),
            Value::Float(data) => Some(data.to_string()),
            Value::Integer(data) => Some(data.to_string()),
            Value::String(data) => Some(data.to_owned()),
            Value::Array(data) => {
                if data.is_empty() {
                    return None;
                }
                let mut result = String::from("");
                for (index, value) in data.iter().enumerate() {
                    let Some(inner) = Self::get_string_from_value(value) else {
              return None;
            };
                    result += &inner;
                    if index + 1 < data.len() {
                        result += ";";
                    }
                }
                Some(result)
            }
            _ => None,
        }
    }

    pub fn build(mut self) -> HashMap<String, String> {
        // TOML, CONSOLE, ENV
        let mut result: HashMap<String, String> = HashMap::new();
        let matches = self.get_matches();
        let toml = self.get_toml();
        for mut setting in self.data {
            if let Some(group) = &setting.section {
                setting.env = format!("{}_{}", group.to_uppercase(), setting.env);
            }
            if let Some(prefix) = &self.prefix {
                setting.env = format!("{}_{}", prefix, setting.env);
            }
            if let Ok(value) = std::env::var(&setting.env) {
                result.insert(setting.id, value);   
            } else if let Some(value) = Self::get_from_matches(&setting, &matches) {
                result.insert(setting.id, value);
            } else if toml.is_some() {
                let tmp = toml.as_ref().unwrap();
                if let Some(group) = &setting.section {
                    if let Some(value) = tmp.get(group) {
                        if value.is_table() {
                            if let Some(inner) = value.get(&setting.id) {
                                if let Some(param_value) = Self::get_string_from_value(inner) {
                                    result.insert(setting.id, param_value);
                                }
                            }
                        }
                    }
                } else {
                    if let Some(value) = tmp.get(&setting.id) {
                        if let Some(value) = Self::get_string_from_value(value) {
                            result.insert(setting.id, value);
                        }
                    }
                }
            }
        }
        result
    }
}
