# rek0n-chunk

Part of [rek0n](https://github.com/K48N/rek0n). Shared chunk types between the parser and embed crates.

## Overview

`rek0n-chunk` defines the canonical chunk types for the rek0n parse-to-embed pipeline. Every downstream crate uses these names directly:

| Type | Stage | Contains |
|------|-------|----------|
| `ParsedChunk` | After parsing | kind, name, text, line range, `has_error` |
| `IndexedChunk` | Before embedding | same fields plus `file_path` |

`ChunkKind` is the shared enum for functions, structs, impl blocks, and the rest.

## How it works

1. **rek0n-parser** emits `ParsedChunk` values (no file path; syntax metadata included).
2. **rek0n-embed** accepts `IndexedChunk` values (path attached at index time).
3. Bridge with `IndexedChunk::from_parsed(path, &parsed)` or embed's `try_from_parser_chunk`.
4. Shared validation: `validate_file_path`, `validate_input_text_length`.

## Design

**One vocabulary.** Parser and embed used to alias both stages as `SemanticChunk`. That hid the pipeline boundary and drifted in docs. Two explicit types, one crate.

**Path belongs at index time.** The parser reads a buffer; the caller knows which file was parsed.

**Small dependency surface.** Optional serde only. No tree-sitter, Candle, or LanceDB.

## Usage

Parser output:

```rust
use rek0n_parser::{parse_file, ParsedChunk};
```

Embed input:

```rust
use rek0n_chunk::{IndexedChunk, ParsedChunk};
use rek0n_embed::{try_from_parser_chunk, IndexedChunk};

let indexed = try_from_parser_chunk("src/lib.rs", &parsed)?;
indexed.validate()?;
```

Local development expects this repo as a sibling of rek0n-parser and rek0n-embed:

```toml
rek0n-chunk = { path = "../rek0n-chunk", version = "0.1.0" }
```

## Known gaps

- No schema version field on chunks yet. Adding one would need a coordinated bump across parser, embed, and db.
- Embed maps `ChunkError` into `EmbedError` at the boundary.

## License

MIT
