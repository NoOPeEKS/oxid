use crate::types::{
    ClientCapabilities, ClientInfo, HoverClientCapabilities, InitializeParams, MarkupKind,
    TextDocumentClientCapabilities,
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
                        .hover(
                            HoverClientCapabilities::builder()
                                .dynamic_registration(false)
                                .content_format(vec![MarkupKind::PlainText])
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build()
}
