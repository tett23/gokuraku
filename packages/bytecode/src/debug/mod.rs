pub struct DebugSymbol {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub file: Option<String>,
    pub raw_symbol: Option<String>,
}
