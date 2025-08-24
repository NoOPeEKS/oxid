use core::panic;

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

#[derive(Debug, Serialize, Deserialize, Default)]
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
    pub trace: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub workspace: Option<WorkspaceClientCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document: Option<TextDocumentClientCapabilities>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub window: Option<WindowClientCapabilities>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub general: Option<GeneralClientCapabilities>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub experimental: Option<Value>,
}

// For now we only cover hover capabilities (easy)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverClientCapabilities>,
    // pub completion: Option<CompletionClientCapabilities>,
    // pub synchronization: Option<TextDocumentSyncClientCapabilities>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_format: Option<Vec<MarkupKind>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
    PlainText,
    Markdown,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: String,
    pub version: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentPositionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

pub type DocumentSelector = Vec<DocumentFilter>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotation {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_confirmation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub type ChangeAnnotationIdentifier = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotatedTextEdit {
    pub range: Range,
    pub new_text: String,
    pub annotation_id: ChangeAnnotationIdentifier,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentEdit {
    pub text_document: TextDocumentIdentifier,
    pub edits: Vec<TextEdit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_selection_range: Option<Range>,
    pub target_uri: String,
    pub target_range: Range,
    pub target_selection_range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDescription {
    pub href: String
}
