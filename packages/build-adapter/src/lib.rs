use anyhow::Result;
use gokuraku_config::GokurakuConfigInstance;
use prose_ir::Document;

pub struct BuildArtifact {
    pub name: String,
    pub content: Vec<u8>,
}

pub trait BuildAdapter {
    fn name(&self) -> String;
    fn build(
        &self,
        config: &GokurakuConfigInstance,
        documents: &[(String, Document)],
    ) -> Result<BuildArtifact>;
}

pub trait BuildAdapterInitializable {
    fn adapter_name() -> String;
    fn init(&mut self, config: &Option<String>) -> Result<()>;
}
