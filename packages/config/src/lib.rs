use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GokurakuConfig {
    pub index: IndexTree,
    pub formats: Option<Vec<String>>,
    pub output: Option<PathBuf>,
    pub adapters: Vec<BuildAdapterConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildAdapterConfig {
    pub name: String,
    pub options: Option<String>,
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
            adapters: vec![BuildAdapterConfig {
                name: "txt".to_string(),
                options: None,
            }],
        }
    }
}

#[derive(Debug)]
pub struct GokurakuConfigInstance {
    pub env: String,
    pub index: IndexTree,
    pub formats: Vec<String>,
    pub output: Option<PathBuf>,
    pub input: Option<IndexTree>,
    pub adapters: Vec<BuildAdapterConfig>,
}

impl GokurakuConfigInstance {
    pub fn index(&self) -> &IndexTree {
        match self {
            Self {
                input: Some(input), ..
            } => input,
            Self { index, .. } => index,
        }
    }
}

impl TryFrom<GokurakuConfig> for GokurakuConfigInstance {
    type Error = anyhow::Error;

    fn try_from(conf: GokurakuConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            env: "development".to_string(),
            index: conf.index,
            formats: conf.formats.unwrap_or(Vec::new()),
            output: conf.output,
            input: None,
            adapters: conf.adapters,
        })
    }
}

pub struct CLIArgs {
    pub env: String,
    pub output: Option<PathBuf>,
    pub formats: Option<Vec<String>>,
    pub input: Option<String>,
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
            input: args.input.map(IndexTree::Leaf),
            adapters: vec![BuildAdapterConfig {
                name: "txt".to_string(),
                options: None,
            }],
        })
    }
}
