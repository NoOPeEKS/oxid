use core::panic;
use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};

//----------------BASE LSP MESSAGES----------------
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: i64,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: i64,
    pub result: Option<serde_json::Value>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

//--------------PARAMS AND SUBPARAMS TYPES----------------

#[derive(Debug, Serialize, Deserialize, Default, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_info: Option<ClientInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_options: Option<serde_json::Value>,

    pub capabilities: ClientCapabilities,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<TraceValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document: Option<TextDocumentClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<GeneralClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
}

// For now we only cover hover capabilities (easy)
#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CompletionClientCapabilities>,
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,
    // pub signature_help: Option<SignatureHelpClientCapabilities>,
    // pub declaration: Option<DeclarationClientCapabilities>,
    // pub definition: Option<DefinitionClientCapabilities>,
    // pub type_definition: Option<TypeDefinitionClientCapabilities>,
    // pub implementation: Option<ImplementationClientCapabilities>,
    // pub references: Option<ReferenceClientCapabilities>,
    // pub document_highlight: Option<DocumentHighlightClientCapabilities>,
    // pub document_symbol: Option<DocumentSymbolClientCapabilities>,
    // pub code_action: Option<CodeActionClientCapabilities>,
    // pub code_lens: Option<CodeLensClientCapabilities>,
    // pub document_link: Option<DocumentLinkClientCapabilities>,
    // pub color_provider: Option<DocumentColorClientCapabilities>,
    // pub formatting: Option<DocumentFormattingClientCapabilities>,
    // pub range_formatting: Option<DocumentRangeFormattingClientCapabilities>,
    // pub on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,
    // pub rename: Option<RenameClientCapabilities>,
    // pub publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,
    // pub folding_range: Option<FoldingRangeClientCapabilities>,
    // pub selection_range: Option<SelectionRangeClientCapabilities>,
    // pub linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,
    // pub call_hierarchy: Option<CallHierarchyClientCapabilities>,
    // pub semantic_tokens: Option<SemanticTokensClientCapabilities>,
    // pub moniker: Option<MonikerClientCapabilities>,
    // pub type_hierarchy: Option<TypeHierarchyClientCapabilities>,
    // pub inline_value: Option<InlineValueClientCapabilities>,
    // pub inlay_hint: Option<InlayHintClientCapabilities>,
    // pub diagnostic: Option<DiagnosticClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save_until: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_save: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_item: Option<CompletionItemCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_item_kind:Option<CompletionItemKindCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_mode: Option<InsertTextMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_list: Option<CompletionListCapability>,
}
#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemKindCapability {
    pub value_set: Option<Vec<CompletionItemKind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

impl From<i32> for CompletionItemKind {
    fn from(value: i32) -> Self {
        match value {
            1 => CompletionItemKind::Text,
            2 => CompletionItemKind::Method,
            3 => CompletionItemKind::Function,
            4 => CompletionItemKind::Constructor,
            5 => CompletionItemKind::Field,
            6 => CompletionItemKind::Variable,
            7 => CompletionItemKind::Class,
            8 => CompletionItemKind::Interface,
            9 => CompletionItemKind::Module,
            10 => CompletionItemKind::Property,
            11 => CompletionItemKind::Unit,
            12 => CompletionItemKind::Value,
            13 => CompletionItemKind::Enum,
            14 => CompletionItemKind::Keyword,
            15 => CompletionItemKind::Snippet,
            16 => CompletionItemKind::Color,
            17 => CompletionItemKind::File,
            18 => CompletionItemKind::Reference,
            19 => CompletionItemKind::Folder,
            20 => CompletionItemKind::EnumMember,
            21 => CompletionItemKind::Constant,
            22 => CompletionItemKind::Struct,
            23 => CompletionItemKind::Event,
            24 => CompletionItemKind::Operator,
            25 => CompletionItemKind::TypeParameter,
            _ => panic!("Invalid CompletionItemKind value: {}", value),
        }
    }
}

impl From<CompletionItemKind> for i32 {
    fn from(kind: CompletionItemKind) -> i32 {
        kind as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_characters_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_format: Option<Vec<MarkupKind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preselect_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_support: Option<TagSupport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_replace_support: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_support: Option<ResolveSupport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_mode_support: Option<InsertTextModeSupport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_details_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextModeSupport {
    pub value_set: Vec<InsertTextMode>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResolveSupport {
    pub properties: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TagSupport {
    pub value_set: Vec<CompletionItemTagKind>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum CompletionItemTagKind {
    Deprecated = 1,
}

impl From<i32> for CompletionItemTagKind {
    fn from(value: i32) -> Self {
        match value {
            1 => CompletionItemTagKind::Deprecated,
            _ => panic!("Invalid CompletionItemTagKind value: {}", value),
        }
    }
}

impl From<CompletionItemTagKind> for i32 {
    fn from(kind: CompletionItemTagKind) -> i32 {
        kind as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionListCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_defaults: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum InsertTextMode {
    AsIs = 1,
    AdjustIndentation = 2,
}

impl From<i32> for InsertTextMode {
    fn from(value: i32) -> Self {
        match value {
            1 => InsertTextMode::AsIs,
            2 => InsertTextMode::AdjustIndentation,
            _ => panic!("Invalid InsertTextMode value: {}", value),
        }
    }
}

impl From<InsertTextMode> for i32 {
    fn from(mode: InsertTextMode) -> i32 {
        mode as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList {
    pub is_incomplete: bool,
    pub items: Vec<CompletionItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_defaults: Option<ItemDefaults>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ItemDefaults {
    pub commit_characters: Option<Vec<String>>,
    pub edit_range: Option<EditRangeKind>,
    pub insert_text_format: Option<InsertTextFormat>,
    pub insert_text_mode: Option<InsertTextMode>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditRangeKind {
    Simple(Range),
    InsertReplace(InsertReplaceRange),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertReplaceRange {
    pub insert: Range,
    pub replace: Range,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_details: Option<CompletionItemLabelDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CompletionItemKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CompletionItemTagKind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<CompletionItemDocumentationKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preselect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<InsertTextFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_mode: Option<InsertTextMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_edit: Option<TextEditKind>, // OR InsertReplaceEdit can also be it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_edit_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_text_edits: Option<Vec<TextEdit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextEditKind {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit)
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InsertReplaceEdit {
    pub new_text: String,
    pub insert: Range,
    pub replace: Range,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemDocumentationKind {
    Simple(String),
    Markup(MarkupContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum InsertTextFormat {
    PlainText = 1,
    Snippet = 2,
}

impl From<i32> for InsertTextFormat {
    fn from(value: i32) -> Self {
        match value {
            1 => InsertTextFormat::PlainText,
            2 => InsertTextFormat::Snippet,
            _ => panic!("Invalid InsertTextFormat value: {}", value),
        }
    }
}

impl From<InsertTextFormat> for i32 {
    fn from(value: InsertTextFormat) -> i32 {
        value as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemLabelDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_format: Option<Vec<MarkupKind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionEncodingKind {
    #[serde(rename = "utf-8")]
    Utf8,
    #[serde(rename = "utf-16")]
    Utf16,
    #[serde(rename = "utf-32")]
    Utf32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentPositionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct DocumentFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

pub type DocumentSelector = Vec<DocumentFilter>;

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotation {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_confirmation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub type ChangeAnnotationIdentifier = String;

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct AnnotatedTextEdit {
    pub range: Range,
    pub new_text: String,
    pub annotation_id: ChangeAnnotationIdentifier,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentEdit {
    pub text_document: TextDocumentIdentifier,
    pub edits: Vec<TextEdit>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_selection_range: Option<Range>,
    pub target_uri: String,
    pub target_range: Range,
    pub target_selection_range: Range,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub range: Range,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosticSeverity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_description: Option<CodeDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DiagnosticTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

impl From<DiagnosticSeverity> for i32 {
    fn from(value: DiagnosticSeverity) -> Self {
        value as i32
    }
}

impl From<i32> for DiagnosticSeverity {
    fn from(value: i32) -> Self {
        match value {
            1 => DiagnosticSeverity::Error,
            2 => DiagnosticSeverity::Warning,
            3 => DiagnosticSeverity::Information,
            4 => DiagnosticSeverity::Hint,
            _ => panic!("Invalid DiagnosticSeverity value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum DiagnosticTag {
    Unnecessary = 1,
    Deprecated = 2,
}

impl From<DiagnosticTag> for i32 {
    fn from(value: DiagnosticTag) -> Self {
        value as i32
    }
}

impl From<i32> for DiagnosticTag {
    fn from(value: i32) -> Self {
        match value {
            1 => DiagnosticTag::Unnecessary,
            2 => DiagnosticTag::Deprecated,
            _ => panic!("Invalid DiagnosticTag value: {}", value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct CodeDescription {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub title: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
    PlainText,
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownClientCapabilities {
    pub parser: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_if_exists: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CreateFile {
    pub kind: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateFileOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct RenameFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_if_exists: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct RenameFile {
    pub kind: String,
    pub old_uri: String,
    pub new_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<RenameFileOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_if_not_exists: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFile {
    pub kind: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DeleteFileOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<DocumentChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_annotations: Option<HashMap<String, ChangeAnnotation>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentChanges {
    TextDocumentEdits(Vec<TextDocumentEdit>),
    Operations(Vec<DocumentChangeOperation>),
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocumentChangeOperation {
    TextDocumentEdit(TextDocumentEdit),
    CreateFile(CreateFile),
    RenameFile(RenameFile),
    DeleteFile(DeleteFile),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_operations: Option<ResourceOperationKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_handling: Option<FailureHandlingKind>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceOperationKind {
    Create,
    Rename,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FailureHandlingKind {
    Abort,
    Transactional,
    Undo,
    TextOnlyTransactional,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TraceValue {
    Off,
    Messages,
    Verbose,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GeneralClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_request_support: Option<StaleRequestSupportClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_expressions: Option<RegularExpressionsClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_encodings: Option<Vec<PositionEncodingKind>>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct RegularExpressionsClientCapabilities {
    pub engine: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct StaleRequestSupportClientCapabilities {
    pub cancel: bool,
    pub retry_on_content_modified: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_edit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_edit: Option<WorkspaceEditClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<WorkspaceSymbolClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command: Option<ExecuteCommandClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_lens: Option<CodeLensWorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_operations: Option<FileOperationsWorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_value: Option<InlineValueWorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlay_hint: Option<InlayHintWorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticWorkspaceClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_pattern_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_kind: Option<SymbolKindCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SymbolKindCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_set: Option<Vec<SymbolKind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

impl From<i32> for SymbolKind {
    fn from(value: i32) -> Self {
        match value {
            1 => SymbolKind::File,
            2 => SymbolKind::Module,
            3 => SymbolKind::Namespace,
            4 => SymbolKind::Package,
            5 => SymbolKind::Class,
            6 => SymbolKind::Method,
            7 => SymbolKind::Property,
            8 => SymbolKind::Field,
            9 => SymbolKind::Constructor,
            10 => SymbolKind::Enum,
            11 => SymbolKind::Interface,
            12 => SymbolKind::Function,
            13 => SymbolKind::Variable,
            14 => SymbolKind::Constant,
            15 => SymbolKind::String,
            16 => SymbolKind::Number,
            17 => SymbolKind::Boolean,
            18 => SymbolKind::Array,
            19 => SymbolKind::Object,
            20 => SymbolKind::Key,
            21 => SymbolKind::Null,
            22 => SymbolKind::EnumMember,
            23 => SymbolKind::Struct,
            24 => SymbolKind::Event,
            25 => SymbolKind::Operator,
            26 => SymbolKind::TypeParameter,
            _ => panic!("Invalid SymbolKind value: {}", value),
        }
    }
}

impl From<SymbolKind> for i32 {
    fn from(kind: SymbolKind) -> i32 {
        kind as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationsWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_create: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_create: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_rename: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_rename: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_delete: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WindowClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_message: Option<ShowMessageRequestClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_document: Option<ShowDocumentClientCapabilities>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageRequestClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action_item: Option<MessageActionItem>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_encoding: Option<PositionEncodingKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document_sync: Option<TextDocumentSyncOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_provider: Option<CompletionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_provider: Option<HoverProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_help_provider: Option<SignatureHelpOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_provider: Option<DeclarationProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_provider: Option<DefinitionProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_definition_provider: Option<TypeDefinitionProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_provider: Option<ImplementationProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references_provider: Option<ReferencesProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_highlight_provider: Option<DocumentHighlightProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_symbol_provider: Option<DocumentSymbolProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_action_provider: Option<CodeActionProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_lens_provider: Option<CodeLensOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_link_provider: Option<DocumentLinkOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_provider: Option<ColorProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_formatting_provider: Option<DocumentFormattingProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_range_formatting_provider: Option<DocumentRangeFormattingProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_provider: Option<RenameProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folding_range_provider: Option<FoldingRangeProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command_provider: Option<ExecuteCommandOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_range_provider: Option<SelectionRangeProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_editing_range_provider: Option<LinkedEditingRangeProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_hierarchy_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_tokens_provider: Option<SemanticTokensOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moniker_provider: Option<MonikerProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_hierarchy_provider: Option<TypeHierarchyProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_value_provider: Option<InlineValueProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlay_hint_provider: Option<InlayHintOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_provider: Option<DiagnosticProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_symbol_provider: Option<WorkspaceSymbolProviderCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceServerCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_close: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<TextDocumentSyncKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_save_wait_until: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<TextDocumentSyncSaveOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSyncSaveOptions {
    Simple(bool),
    Options(SaveOptions)
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SaveOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_text: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(from = "i32", into = "i32")]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

impl From<i32> for TextDocumentSyncKind {
    fn from(value: i32) -> Self {
        match value {
            0 => TextDocumentSyncKind::None,
            1 => TextDocumentSyncKind::Full,
            2 => TextDocumentSyncKind::Incremental,
            _ => panic!("Invalid TextDocumentSyncKind value: {}", value),
        }
    }
}

impl From<TextDocumentSyncKind> for i32 {
    fn from(kind: TextDocumentSyncKind) -> i32 {
        kind as i32
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_commit_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_item: Option<CompletionOptionsCompletionItem>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptionsCompletionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_details_support: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverProviderCapability {
    Simple(bool),
    Options(HoverOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct HoverOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrigger_characters: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclarationProviderCapability {
    Simple(bool),
    Options(DeclarationOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionProviderCapability {
    Simple(bool),
    Options(DefinitionOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeDefinitionProviderCapability {
    Simple(bool),
    Options(TypeDefinitionOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImplementationProviderCapability {
    Simple(bool),
    Options(ImplementationOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReferencesProviderCapability {
    Simple(bool),
    Options(ReferenceOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentHighlightProviderCapability {
    Simple(bool),
    Options(DocumentHighlightOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentSymbolProviderCapability {
    Simple(bool),
    Options(DocumentSymbolOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionProviderCapability {
    Simple(bool),
    Options(CodeActionOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorProviderCapability {
    Simple(bool),
    Options(DocumentColorOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentFormattingProviderCapability {
    Simple(bool),
    Options(DocumentFormattingOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentRangeFormattingProviderCapability {
    Simple(bool),
    Options(DocumentRangeFormattingOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingOptions {
    pub first_trigger_character: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_trigger_character: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameProviderCapability {
    Simple(bool),
    Options(RenameOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct RenameOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_provider: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldingRangeProviderCapability {
    Simple(bool),
    Options(FoldingRangeOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    pub commands: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectionRangeProviderCapability {
    Simple(bool),
    Options(SelectionRangeOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedEditingRangeProviderCapability {
    Simple(bool),
    Options(LinkedEditingRangeOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensOptions {
    pub legend: SemanticTokensLegend,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<SemanticTokensFull>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensLegend {
    pub token_types: Vec<String>,
    pub token_modifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensFull {
    pub delta: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonikerProviderCapability {
    Simple(bool),
    Options(MonikerOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct MonikerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeHierarchyProviderCapability {
    Simple(bool),
    Options(TypeHierarchyOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineValueProviderCapability {
    Simple(bool),
    Options(InlineValueOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DiagnosticProviderCapability {
    Options(DiagnosticOptions),
    RegistrationOptions(DiagnosticRegistrationOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    pub inter_file_dependencies: bool,
    pub workspace_diagnostics: bool,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRegistrationOptions {
    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,
    #[serde(flatten)]
    pub diagnostic_options: DiagnosticOptions,
    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct StaticRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolProviderCapability {
    Simple(bool),
    Options(WorkspaceSymbolOptions),
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_provider: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_operations: Option<FileOperationsServerCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_notifications: Option<ChangeNotificationsCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChangeNotificationsCapability {
    Simple(bool),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationsServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_create: Option<FileOperationRegistrationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_create: Option<FileOperationRegistrationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_rename: Option<FileOperationRegistrationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_rename: Option<FileOperationRegistrationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_delete: Option<FileOperationRegistrationOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_delete: Option<FileOperationRegistrationOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationRegistrationOptions {
    pub filters: Vec<FileOperationFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationFilter {
    pub scheme: Option<String>,
    pub pattern: FileOperationPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationPattern {
    pub glob: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<FileOperationPatternOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileOperationPatternKind {
    File,
    Folder,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationPatternOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Hover {
    pub contents: MarkupContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentContentChangeEvent {
    Full(FullTextDocumentContentChangeEvent),
    Incremental(IncrementalTextDocumentContentChangeEvent)
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullTextDocumentContentChangeEvent {
    pub text: String
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncrementalTextDocumentContentChangeEvent {
    pub range: Range,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_length: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DidSaveTextDocumentParams {
    pub text_document: TextDocumentIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
