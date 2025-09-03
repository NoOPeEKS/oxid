use std::vec;

use crate::types::{
    ClientCapabilities, ClientInfo, CompletionClientCapabilities, CompletionItemCapability,
    CompletionItemTagKind, HoverClientCapabilities, InitializeParams, InsertTextMode,
    InsertTextModeSupport, MarkupKind, PublishDiagnosticsClientCapabilities, TagSupport,
    TextDocumentClientCapabilities, TextDocumentSyncClientCapabilities,
};

/// Defines the capabilities supported by Oxid's LSP Client
pub fn get_client_capabilities() -> InitializeParams {
    InitializeParams::builder()
        .client_info(
            ClientInfo::builder()
                .name(String::from("oxid"))
                .version(String::from("0.1.0"))
                .build(),
        )
        .root_uri(String::from(
            "file:///home/beri/dev/oxid/oxid-lsp/src/lib.rs",
        ))
        .capabilities(
            ClientCapabilities::builder()
                .text_document(
                    TextDocumentClientCapabilities::builder()
                        .synchronization(
                            TextDocumentSyncClientCapabilities::builder()
                                .did_save(true)
                                .will_save(false)
                                .will_save_until(false)
                                .dynamic_registration(false)
                                .build(),
                        )
                        .hover(
                            HoverClientCapabilities::builder()
                                .dynamic_registration(false)
                                .content_format(vec![MarkupKind::PlainText])
                                .build(),
                        )
                        .completion(
                            CompletionClientCapabilities::builder()
                                .dynamic_registration(false)
                                .completion_item(
                                    CompletionItemCapability::builder()
                                        .snippet_support(false)
                                        .documentation_format(vec![MarkupKind::PlainText])
                                        .deprecated_support(true)
                                        .preselect_support(true)
                                        .tag_support(
                                            TagSupport::builder()
                                                .value_set(vec![CompletionItemTagKind::Deprecated])
                                                .build(),
                                        )
                                        .insert_replace_support(true)
                                        .insert_text_mode_support(
                                            InsertTextModeSupport::builder()
                                                .value_set(vec![
                                                    InsertTextMode::AsIs,
                                                    InsertTextMode::AdjustIndentation,
                                                ])
                                                .build(),
                                        )
                                        .label_details_support(true)
                                        // .commit_characters_support()
                                        // .resolve_support(todo!())
                                        .build(),
                                )
                                .insert_text_mode(InsertTextMode::AsIs)
                                // .context_support()
                                // .completion_item_kind()
                                // .completion_list()
                                .build(),
                        )
                        .publish_diagnostics(
                            PublishDiagnosticsClientCapabilities::builder()
                                .data_support(false)
                                .tag_support(
                                    TagSupport::builder()
                                        .value_set(vec![CompletionItemTagKind::Deprecated])
                                        .build(),
                                )
                                .code_description_support(true)
                                .version_support(false)
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build()
}
