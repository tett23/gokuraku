use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GokurakuConfig {
    pub index: IndexTree,
    pub formats: Option<Vec<String>>,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexTree {
    Root(Vec<IndexTree>),
    Node(String, Vec<IndexTree>),
    Leaf(String),
}

impl GokurakuConfig {
    pub fn from_toml_str(toml: &str) -> Result<Self> {
        toml::from_str::<GokurakuConfig>(toml).context("config parse error")
    }

    pub fn as_toml_str(&self) -> Result<String> {
        toml::to_string_pretty(self).context("")
    }
}

impl Default for GokurakuConfig {
    fn default() -> Self {
        Self {
            index: IndexTree::Root(Vec::new()),
            formats: None,
            output: None,
        }
    }
}

#[derive(Debug)]
pub struct GokurakuConfigInstance {
    pub env: String,
    pub index: IndexTree,
    pub formats: Vec<String>,
    pub output: Option<PathBuf>,
}

impl TryFrom<GokurakuConfig> for GokurakuConfigInstance {
    type Error = anyhow::Error;

    fn try_from(conf: GokurakuConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            env: "development".to_string(),
            index: conf.index,
            formats: conf.formats.unwrap_or(Vec::new()),
            output: conf.output,
        })
    }
}

pub struct CLIArgs {
    pub env: String,
    pub output: Option<PathBuf>,
    pub formats: Option<Vec<String>>,
}

impl TryFrom<(GokurakuConfig, CLIArgs)> for GokurakuConfigInstance {
    type Error = anyhow::Error;

    fn try_from((conf, args): (GokurakuConfig, CLIArgs)) -> Result<Self, Self::Error> {
        Ok(Self {
            env: args.env,
            index: conf.index,
            formats: [conf.formats, args.formats]
                .into_iter()
                .flatten()
                .last()
                .unwrap_or(Vec::new()),
            output: [conf.output, args.output].into_iter().flatten().last(),
        })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
