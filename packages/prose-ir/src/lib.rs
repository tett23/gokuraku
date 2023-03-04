use std::fmt::Debug;

#[derive(Debug)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum ProseIR {
    Document(Document),
    Block(Block),
    Inline(Inline),
}

pub enum Block {
    Paragraph(Vec<Inline>),
    EmptyLine,
    ThemanticBreak,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paragraph(value) => {
                write!(f, "Paragraph(\n\t")?;
                value.iter().try_for_each(|item| item.fmt(f))?;
                write!(f, "\n)")
            }
            Self::ThemanticBreak => write!(f, "ThemanticBreak"),
            Self::EmptyLine => writeln!(f),
        }
    }
}

pub enum Inline {
    Text(String),
    Number(String),
    Expr(String),
}

impl Debug for Inline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(value) if matches!(value.as_str(), "\r\n" | "\n" | "\r") => {
                write!(f, "")
            }
            Self::Text(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "Number(##{value}##)"),
            Self::Expr(value) => write!(f, "Expr({{{value}}})"),
        }
    }
}
