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

pub struct Header;

pub enum ContentType {
    TextHtml,
    ApplicationJson
}

