use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: i64,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
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

//--------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Default)]
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
pub struct ClientInfo {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Deserialize)]
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
