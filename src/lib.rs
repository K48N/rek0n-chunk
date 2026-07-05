//! Shared chunk types for the rek0n parse-to-embed pipeline.
//!
//! Canonical types:
//! - [`ParsedChunk`]: parser output (no `file_path`, includes `has_error`)
//! - [`IndexedChunk`]: embed input (`file_path` attached, ready to vectorize)

mod error;
mod indexed;
mod kind;
mod limits;
mod parsed;
mod splitter;

pub use error::ChunkError;
pub use indexed::IndexedChunk;
pub use kind::ChunkKind;
pub use limits::{
    validate_file_path, validate_input_text_length, MAX_FILE_PATH_LEN, MAX_INPUT_TEXT_LEN,
};
pub use parsed::ParsedChunk;
pub use splitter::{SemanticSplitter, DEFAULT_MAX_TRANSFORMER_TOKENS};
