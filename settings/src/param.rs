use std::fs;
use std::hash::Hash;
use std::hash::Hasher;

use clap::{Arg, ArgAction, ArgMatches, Command};
use linked_hash_set::LinkedHashSet;
use toml::Value;

use crate::any::AnyValue;
use crate::any::SettingsMap;
use crate::utils::check_if_valid_env;
use crate::Error;

#[derive(Hash, PartialEq, Eq)]
pub enum ParamType {
    Enum(Vec<String>),
    RequiredSet,
    Flag,
    Set,
    Multivalued,
}

pub struct SettingSchemaBuilder {
    id: String,
    short: Option<char>,
    help: Option<String>,
    param_type: Option<ParamType>,
    section: Option<String>,
    default: Option<String>,
    group_prefix: Option<String>,
    group_description: Option<String>,
    hidden: bool,
}

impl SettingSchemaBuilder {
    pub fn new<T: Into<String>>(id: T) -> Result<Self, Error> {
        let id: String = id.into();
        let id = id.replace("-", "_");
        if id.is_empty() {
            return Err(Error::EmptyString);
        }
        if !check_if_valid_env(&id) {
            return Err(Error::InvalidStringForEnv(id));
        }
        Ok(Self {
            id,
            short: None,
            help: None,
            param_type: None,
            section: None,
            group_prefix: None,
            group_description: None,
            hidden: false,
            default: None,
        })
    }

    pub fn build(mut self) -> SettingSchema {
        let id = self.id;
        SettingSchema {
            id: id.clone(),
            short: self.short.take(),
            env: id.clone().to_uppercase(),
            param_type: self.param_type.unwrap_or(ParamType::Set),
            help: self.help.unwrap_or(id),
            hidden: self.hidden,
            section: self.section,
            group_prefix: self.group_prefix,
            group_description: self.group_description,
            default: self.default,
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

    pub fn help<T: Into<String>>(mut self, value: T) -> Self {
        self.help = Some(value.into());
        self
    }

    pub fn section<T: Into<String>>(mut self, value: T) -> Self {
        self.section = Some(value.into());
        self
    }

    pub fn with_default<T: Into<String>>(mut self, value: T) -> Self {
        self.default = Some(value.into());
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
    env: String,
    param_type: ParamType,
    help: String,
    hidden: bool,
    default: Option<String>,
    pub(crate) section: Option<String>,
    pub(crate) group_prefix: Option<String>,
    pub(crate) group_description: Option<String>,
}

impl PartialEq for SettingSchema {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for SettingSchema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl SettingSchema {
    pub fn to_arg(&self) -> Arg {
        let id = if let Some(group_prefix) = &self.group_prefix {
            format!("{}.{}", group_prefix, self.id)
        } else {
            self.id.clone()
        };
        let mut result = Arg::new(id.clone());
        if let Some(short) = self.short {
            result = result.short(short);
        };
        if let Some(section) = &self.section {
            let section = if let Some(description) = &self.group_description {
                format!("{}({})", section, description)
            } else {
                section.clone()
            };
            result = result.help_heading(section);
        };
        let result = match &self.param_type {
            ParamType::Enum(data) => result.value_parser(data.clone()),
            ParamType::RequiredSet => result.required(true).action(ArgAction::Set),
            ParamType::Flag => result.action(ArgAction::SetTrue),
            ParamType::Set => result.action(ArgAction::Set),
            ParamType::Multivalued => result.action(ArgAction::Append),
        };
        let result = result
            .long(id.clone())
            .help(self.help.clone())
            .hide(self.hidden);
        if let Some(default) = &self.default {
            result.default_value(default)
        } else {
            result
        }
    }
}

pub struct ConfigGenerator {
    data: LinkedHashSet<SettingSchema>,
    toml_filename: Option<String>,
    program_name: Option<String>,
    author: Option<String>,
    about: Option<String>,
    usage: Option<String>,
    version: Option<String>,
    prefix: Option<String>,
}

impl ConfigGenerator {
    pub fn new() -> Self {
        Self {
            data: LinkedHashSet::new(),
            toml_filename: None,
            program_name: None,
            author: None,
            about: None,
            usage: None,
            prefix: None,
            version: None,
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

    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn group<T, S, D>(
        mut self,
        group: T,
        prefix: Option<S>,
        description: Option<D>,
        settings: Vec<SettingSchema>,
    ) -> Result<Self, Error>
    where
        T: Into<String>,
        S: Into<String>,
        D: Into<String>,
    {
        let group = group.into();
        if !check_if_valid_env(&group) {
            return Err(Error::InvalidStringForEnv(group));
        }
        let description = if let Some(description) = description {
            Some(description.into())
        } else {
            None
        };
        let prefix = if let Some(data) = prefix {
            let data = data.into();
            if !check_if_valid_env(&data) {
                return Err(Error::InvalidStringForEnv(data));
            }
            Some(data)
        } else {
            None
        };
        settings.into_iter().for_each(|mut s| {
            s.section = Some(group.clone());
            s.group_prefix = prefix.clone();
            s.group_description = description.clone();
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
            .version(self.version.take().unwrap_or("0.1.0".into()))
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

    fn get_from_matches(setting: &SettingSchema, matches: &ArgMatches) -> Option<AnyValue> {
        let id = if let Some(prefix) = &setting.group_prefix {
            format!("{}.{}", prefix, setting.id)
        } else {
            setting.id.clone()
        };
        if let Some(_) = matches.value_source(&id) {
            match &setting.param_type {
                ParamType::Flag => {
                    Some(AnyValue::new(matches.get_one::<bool>(&id).unwrap().clone().to_string()))
                }
                ParamType::Multivalued => {
                    let result: Vec<String> = matches.get_many(&id).unwrap().cloned().collect();
                    Some(AnyValue::new(result))
                }
                _ => Some(AnyValue::new(
                    matches.get_one::<String>(&id).unwrap().clone(),
                )),
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

    pub fn build(mut self) -> SettingsMap {
        // TOML, CONSOLE, ENV
        let mut result = SettingsMap::new();
        let matches = self.get_matches();
        let toml = self.get_toml();
        for mut setting in self.data {
            if let Some(group) = &setting.group_prefix {
                setting.env = format!("{}_{}", group.to_uppercase(), setting.env);
            }
            if let Some(prefix) = &self.prefix {
                setting.env = format!("{}_{}", prefix, setting.env);
            }
            if let Ok(value) = std::env::var(&setting.env) {
                if let ParamType::Multivalued = setting.param_type {
                    let value = value.split(";");
                    result.insert(
                        setting.id,
                        value.map(|s| String::from(s)).collect::<Vec<String>>(),
                    );
                } else {
                    result.insert(setting.id, value);
                }
            } else if let Some(value) = Self::get_from_matches(&setting, &matches) {
                result.insert_raw(setting.id, value);
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
