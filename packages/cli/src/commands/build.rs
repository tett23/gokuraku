use anyhow::{anyhow, Result};
use build_adapter::{BuildAdapter, BuildAdapterInitializable};
use build_adapter_txt::BuildAdapterTxt;
use gokuraku_config::{GokurakuConfigInstance, IndexTree, IndexTree::*};
use parser::ast;
use std::fs;

pub fn build(conf: &GokurakuConfigInstance) -> Result<()> {
    let docs = parse_index_tree(conf.index())?;

    adapters(conf)?
        .iter()
        .map(|adapter| adapter.build(conf, &docs))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .for_each(|artifact| {
            println!("{}", std::str::from_utf8(&artifact.content).unwrap());
        });

    Ok(())
}

fn adapters(conf: &GokurakuConfigInstance) -> Result<Vec<Box<dyn BuildAdapter>>> {
    let ret = conf
        .adapters
        .iter()
        .map(|adapter_conf| -> Result<Box<dyn BuildAdapter>> {
            match adapter_conf.name.as_str() {
                "txt" => Ok(Box::new({
                    let mut adapter = BuildAdapterTxt::default();
                    adapter.init(&adapter_conf.options)?;

                    adapter
                })),
                name => Err(anyhow!("unknown adapter {name}")),
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|adapter| conf.formats.iter().any(|name| name == &adapter.name()))
        .collect::<Vec<_>>();

    Ok(ret)
}

fn parse_index_tree(tree: &IndexTree) -> Result<Vec<(String, ast::Document)>> {
    match tree {
        Root(nodes) => Ok(nodes
            .iter()
            .map(parse_index_tree)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect()),
        Node(path, nodes) => Ok(vec![(path.to_owned(), read_and_parse(path)?)]
            .into_iter()
            .chain(
                nodes
                    .iter()
                    .map(parse_index_tree)
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten(),
            )
            .collect()),
        Leaf(path) => read_and_parse(path).map(|doc| vec![(path.to_owned(), doc)]),
    }
}

fn read_and_parse(path: &str) -> Result<ast::Document> {
    parser::parse(&fs::read_to_string(path)?)
}
