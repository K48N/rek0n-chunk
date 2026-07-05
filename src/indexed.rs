use crate::error::ChunkError;
use crate::kind::ChunkKind;
use crate::limits::{validate_file_path, validate_input_text_length};
use crate::parsed::ParsedChunk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedChunk {
    pub kind: ChunkKind,
    pub name: Option<String>,
    pub text: String,
    pub start_line: usize,
    pub end_line: usize,
    pub file_path: String,
}

impl IndexedChunk {
    pub fn line_range(&self) -> String {
        format!("{}:{}", self.start_line, self.end_line)
    }

    pub fn from_parsed(
        file_path: impl Into<String>,
        parsed: &ParsedChunk,
    ) -> Result<Self, ChunkError> {
        if parsed.has_error {
            return Err(ChunkError::HasSyntaxErrors);
        }
        Ok(Self {
            kind: parsed.kind,
            name: parsed.name.clone(),
            text: parsed.text.clone(),
            start_line: parsed.start_line,
            end_line: parsed.end_line,
            file_path: file_path.into(),
        })
    }

    pub fn from_parser_parts(
        file_path: impl Into<String>,
        parser_kind: &str,
        name: Option<String>,
        text: impl Into<String>,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        Self {
            kind: ChunkKind::from_parser_kind(parser_kind),
            name,
            text: text.into(),
            start_line,
            end_line,
            file_path: file_path.into(),
        }
    }

    pub fn try_from_parser_parts(
        file_path: impl Into<String>,
        parser_kind: &str,
        name: Option<String>,
        text: impl Into<String>,
        start_line: usize,
        end_line: usize,
        has_error: bool,
    ) -> Result<Self, ChunkError> {
        if has_error {
            return Err(ChunkError::HasSyntaxErrors);
        }
        Ok(Self::from_parser_parts(
            file_path,
            parser_kind,
            name,
            text,
            start_line,
            end_line,
        ))
    }

    pub fn validate(&self) -> Result<(), ChunkError> {
        if self.text.trim().is_empty() {
            return Err(ChunkError::EmptyText);
        }
        validate_file_path(&self.file_path)?;
        if self.end_line < self.start_line {
            return Err(ChunkError::InvertedLineRange {
                start_line: self.start_line,
                end_line: self.end_line,
            });
        }
        validate_input_text_length(&self.text)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::ChunkKind;

    #[test]
    fn from_parsed_rejects_error_flagged_chunks() {
        let parsed = ParsedChunk {
            kind: ChunkKind::Function,
            name: Some("broken".into()),
            text: "fn broken() {}".into(),
            start_line: 1,
            end_line: 1,
            has_error: true,
        };
        assert!(matches!(
            IndexedChunk::from_parsed("src/a.rs", &parsed),
            Err(ChunkError::HasSyntaxErrors)
        ));
    }
}
