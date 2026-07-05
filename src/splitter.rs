use crate::error::ChunkError;
use crate::parsed::ParsedChunk;

/// Hard ceiling for transformer token windows (all-MiniLM-L6-v2 and peers).
pub const DEFAULT_MAX_TRANSFORMER_TOKENS: usize = 512;

/// Conservative chars-per-token estimate for Rust source prior to tokenizer pass.
const CHARS_PER_TOKEN_ESTIMATE: usize = 3;

/// Fractures parser output so every sub-chunk fits under the embedder token ceiling.
#[derive(Debug, Clone)]
pub struct SemanticSplitter {
    max_tokens: usize,
}

impl Default for SemanticSplitter {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticSplitter {
    pub fn new() -> Self {
        Self {
            max_tokens: DEFAULT_MAX_TRANSFORMER_TOKENS,
        }
    }

    pub fn with_max_tokens(max_tokens: usize) -> Self {
        Self { max_tokens }
    }

    pub fn max_tokens(&self) -> usize {
        self.max_tokens
    }

    pub fn estimate_tokens(&self, text: &str) -> usize {
        text.len().div_ceil(CHARS_PER_TOKEN_ESTIMATE).max(1)
    }

    pub fn max_chars(&self) -> usize {
        self.max_tokens * CHARS_PER_TOKEN_ESTIMATE
    }

    /// Split `chunk` into one or more token-safe sub-chunks, preserving metadata.
    pub fn split_chunk(&self, chunk: &ParsedChunk) -> Result<Vec<ParsedChunk>, ChunkError> {
        if chunk.text.trim().is_empty() {
            return Err(ChunkError::EmptyText);
        }

        if self.estimate_tokens(&chunk.text) <= self.max_tokens {
            return Ok(vec![chunk.clone()]);
        }

        let max_chars = self.max_chars();
        let lines: Vec<&str> = chunk.text.lines().collect();
        if lines.is_empty() {
            return Ok(vec![chunk.clone()]);
        }

        let mut sub_chunks = Vec::new();
        let mut window_start = 0usize;
        let mut buffer = String::new();

        for (index, line) in lines.iter().enumerate() {
            let candidate = if buffer.is_empty() {
                (*line).to_string()
            } else {
                format!("{buffer}\n{line}")
            };

            if candidate.len() > max_chars && !buffer.is_empty() {
                sub_chunks.push(make_sub_chunk(chunk, &buffer, window_start, index - 1));
                window_start = index;
                buffer = (*line).to_string();
            } else if candidate.len() > max_chars {
                for piece in split_oversized_line(line, max_chars) {
                    sub_chunks.push(make_sub_chunk(chunk, &piece, index, index));
                }
                window_start = index + 1;
                buffer.clear();
            } else {
                buffer = candidate;
            }
        }

        if !buffer.is_empty() {
            sub_chunks.push(make_sub_chunk(
                chunk,
                &buffer,
                window_start,
                lines.len().saturating_sub(1),
            ));
        }

        debug_assert!(sub_chunks
            .iter()
            .all(|sub| { self.estimate_tokens(&sub.text) <= self.max_tokens }));

        Ok(sub_chunks)
    }
}

fn make_sub_chunk(
    parent: &ParsedChunk,
    text: &str,
    rel_start: usize,
    rel_end: usize,
) -> ParsedChunk {
    ParsedChunk {
        kind: parent.kind,
        name: parent.name.clone(),
        text: text.to_string(),
        start_line: parent.start_line + rel_start,
        end_line: parent.start_line + rel_end,
        has_error: parent.has_error,
    }
}

fn split_oversized_line(line: &str, max_chars: usize) -> Vec<String> {
    line.as_bytes()
        .chunks(max_chars)
        .map(|chunk| String::from_utf8_lossy(chunk).into_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::ChunkKind;

    #[test]
    fn passes_small_chunks_through() {
        let splitter = SemanticSplitter::new();
        let chunk = ParsedChunk {
            kind: ChunkKind::Function,
            name: Some("main".into()),
            text: "fn main() {}".into(),
            start_line: 1,
            end_line: 1,
            has_error: false,
        };
        let out = splitter.split_chunk(&chunk).expect("split should succeed");
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn fractures_oversized_text() {
        let splitter = SemanticSplitter::with_max_tokens(8);
        let chunk = ParsedChunk {
            kind: ChunkKind::Function,
            name: Some("big".into()),
            text: "x".repeat(512),
            start_line: 10,
            end_line: 10,
            has_error: false,
        };
        let out = splitter.split_chunk(&chunk).expect("split should succeed");
        assert!(out.len() > 1);
        assert!(out
            .iter()
            .all(|sub| splitter.estimate_tokens(&sub.text) <= splitter.max_tokens()));
    }
}
