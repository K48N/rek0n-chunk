#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ChunkKind {
    Function,
    Struct,
    Enum,
    Impl,
    Trait,
    Mod,
    Const,
    Static,
    TypeAlias,
    Macro,
    Use,
    Union,
    ExternCrate,
    Unknown,
}

impl ChunkKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChunkKind::Function => "Function",
            ChunkKind::Struct => "Struct",
            ChunkKind::Enum => "Enum",
            ChunkKind::Impl => "Impl",
            ChunkKind::Trait => "Trait",
            ChunkKind::Mod => "Mod",
            ChunkKind::Const => "Const",
            ChunkKind::Static => "Static",
            ChunkKind::TypeAlias => "TypeAlias",
            ChunkKind::Macro => "Macro",
            ChunkKind::Use => "Use",
            ChunkKind::Union => "Union",
            ChunkKind::ExternCrate => "ExternCrate",
            ChunkKind::Unknown => "Unknown",
        }
    }

    pub fn from_parser_kind(kind: &str) -> Self {
        match kind {
            "Function" | "function" => ChunkKind::Function,
            "Struct" | "struct" => ChunkKind::Struct,
            "Enum" | "enum" => ChunkKind::Enum,
            "Impl" | "impl" => ChunkKind::Impl,
            "Trait" | "trait" => ChunkKind::Trait,
            "Mod" | "mod" => ChunkKind::Mod,
            "Const" | "const" => ChunkKind::Const,
            "Static" | "static" => ChunkKind::Static,
            "TypeAlias" | "type_alias" => ChunkKind::TypeAlias,
            "Macro" | "macro" => ChunkKind::Macro,
            "Use" | "use" => ChunkKind::Use,
            "Union" | "union" => ChunkKind::Union,
            "ExternCrate" | "extern_crate" => ChunkKind::ExternCrate,
            _ => ChunkKind::Unknown,
        }
    }
}

impl std::str::FromStr for ChunkKind {
    type Err = std::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(ChunkKind::from_parser_kind(value))
    }
}
