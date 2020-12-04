use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Headers {
    Connection,
    KeepAlive,
    ProxyAlive,
    ProxyAuthorization,
    TE,
    Trailer,
    TransferEncoding,
    Upgrade,
    ContentType(ContentType),
    AccessControlAllowOrigin(String),
}

#[derive(Debug, Clone)]
pub struct Header(HashMap<String, String>);

#[derive(Debug, Clone)]
pub enum ContentType {
    TextHtml,
    ApplicationJson
}

