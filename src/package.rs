use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub dependencies: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub licenses: Vec<String>,
    pub version: String,
    pub iteration: u32,
    pub sources: Option<Vec<Source>>,
    pub recipie: Option<Recipie>,
}

impl TryFrom<&Path> for Package {
    type Error = ParsingError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let raw = fs::read_to_string(value)?;
        Ok(toml::from_str(&raw)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub url: String,
    pub sha256: String,
}

#[derive(Debug, Deserialize)]
pub struct Recipie {
    pub prepare: Option<Vec<Vec<String>>>,
    pub build: Option<Vec<Vec<String>>>,
    pub test: Option<Vec<Vec<String>>>,
    pub install: Option<Vec<Vec<String>>>,
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError { message: err.to_string() }
    }
}

impl From<toml::de::Error> for ParsingError {
    fn from(err: toml::de::Error) -> Self {
        ParsingError { message: err.to_string() }
    }
}