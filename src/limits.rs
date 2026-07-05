pub const MAX_FILE_PATH_LEN: usize = 4096;
pub const MAX_INPUT_TEXT_LEN: usize = 262_144;

pub fn validate_input_text_length(text: &str) -> Result<(), crate::ChunkError> {
    if text.len() > MAX_INPUT_TEXT_LEN {
        return Err(crate::ChunkError::InputTooLong {
            len: text.len(),
            max: MAX_INPUT_TEXT_LEN,
        });
    }
    Ok(())
}

pub fn validate_file_path(file_path: &str) -> Result<(), crate::ChunkError> {
    if file_path.trim().is_empty() {
        return Err(crate::ChunkError::EmptyFilePath);
    }
    if file_path.len() > MAX_FILE_PATH_LEN {
        return Err(crate::ChunkError::FilePathTooLong {
            len: file_path.len(),
            max: MAX_FILE_PATH_LEN,
        });
    }
    if file_path.chars().any(char::is_control) {
        return Err(crate::ChunkError::FilePathControlChars);
    }
    Ok(())
}
