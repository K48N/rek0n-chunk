#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkError {
    #[error("chunk text must not be empty")]
    EmptyText,

    #[error("file_path must not be empty")]
    EmptyFilePath,

    #[error("input length {len} exceeds limit of {max} characters")]
    InputTooLong { len: usize, max: usize },

    #[error("file_path length {len} exceeds limit of {max}")]
    FilePathTooLong { len: usize, max: usize },

    #[error("file_path must not contain control characters")]
    FilePathControlChars,

    #[error("end_line ({end_line}) must be >= start_line ({start_line})")]
    InvertedLineRange { start_line: usize, end_line: usize },

    #[error("parser flagged chunk with syntax errors")]
    HasSyntaxErrors,
}
