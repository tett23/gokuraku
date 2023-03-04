use anyhow::Result;
use gokuraku_config::{GokurakuConfigInstance, IndexTree, IndexTree::*};
use std::fs;

pub fn build(conf: &GokurakuConfigInstance) -> Result<()> {
    let docs = a(&conf.index)?;

    dbg!(docs);

    Ok(())
}

fn a(tree: &IndexTree) -> Result<Vec<(String, prose_ir::Document)>> {
    match tree {
        Root(nodes) => Ok(nodes
            .iter()
            .map(a)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect()),
        Node(path, nodes) => Ok(vec![(path.to_owned(), read_and_parse(path)?)]
            .into_iter()
            .chain(
                nodes
                    .iter()
                    .map(a)
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten(),
            )
            .collect()),
        Leaf(path) => read_and_parse(path).map(|doc| vec![(path.to_owned(), doc)]),
    }
}

fn read_and_parse(path: &str) -> Result<prose_ir::Document> {
    prose_down::parse(&fs::read_to_string(path)?)
}
