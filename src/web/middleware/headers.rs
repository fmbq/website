use poem::{
    http::header::{CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS},
    middleware::SetHeader,
};

/// Return a middleware that sets some static security headers.
pub fn security_headers() -> SetHeader {
    SetHeader::new()
        .appending(
            CONTENT_SECURITY_POLICY,
            "default-src 'self'; \
            img-src 'self' placeholder.pics; \
            style-src-elem 'self' 'unsafe-inline'; \
            script-src 'self' 'unsafe-inline' 'unsafe-eval'",
        )
        .appending(
            "permissions-policy",
            "geolocation=(), microphone=(), camera=()",
        )
        .appending(REFERRER_POLICY, "strict-origin-when-cross-origin")
        .appending(X_CONTENT_TYPE_OPTIONS, "nosniff")
        .appending("x-frame-options", "DENY")
}
