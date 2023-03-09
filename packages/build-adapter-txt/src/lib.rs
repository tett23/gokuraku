use build_adapter::{BuildAdapter, BuildAdapterInitializable, BuildArtifact};
use parser::ast::{Block, Block::*, Document, Inline, Inline::*};

#[derive(Debug, Default)]
pub struct BuildAdapterTxt();

impl BuildAdapter for BuildAdapterTxt {
    fn name(&self) -> String {
        "txt".to_string()
    }

    fn build(
        &self,
        _config: &gokuraku_config::GokurakuConfigInstance,
        documents: &[(String, Document)],
    ) -> anyhow::Result<build_adapter::BuildArtifact> {
        let content = documents
            .iter()
            .map(|(_, doc)| format_document(doc))
            .collect::<String>();

        Ok(BuildArtifact {
            name: "foo".to_string(),
            content: content.into(),
        })
    }
}

impl BuildAdapterInitializable for BuildAdapterTxt {
    fn adapter_name() -> String {
        "txt".to_string()
    }

    fn init(&mut self, _config: &Option<String>) -> anyhow::Result<()> {
        Ok(())
    }
}

fn format_document(doc: &Document) -> String {
    doc.blocks.iter().map(format_block).collect::<String>()
}

fn format_block(block: &Block) -> String {
    match block {
        EmptyLine => "\n".to_string(),
        Paragraph(inlines) => inlines.iter().map(format_inline).collect::<String>() + "\n",
        ThemanticBreak => "\n---\n".to_string(),
        PdsScript(value) => format!("@{{{value}}}") + "\n",
    }
}

fn format_inline(inline: &Inline) -> String {
    match inline {
        Text(value) => value.to_owned(),
        Number(value) => format!("##{value}##"),
        Expr(_value) => "".to_string(),
    }
}
