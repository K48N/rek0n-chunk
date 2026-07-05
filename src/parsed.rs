use crate::kind::ChunkKind;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParsedChunk {
    pub kind: ChunkKind,
    pub name: Option<String>,
    pub text: String,
    pub start_line: usize,
    pub end_line: usize,
    pub has_error: bool,
}
