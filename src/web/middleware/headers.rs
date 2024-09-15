use poem::middleware::SetHeader;

/// Return a middleware that sets some static security headers.
pub fn security_headers() -> SetHeader {
    SetHeader::new()
        .appending("content-security-policy", "default-src 'self'")
        .appending(
            "permissions-policy",
            "geolocation=(), microphone=(), camera=()",
        )
        .appending("referrer-policy", "strict-origin-when-cross-origin")
        .appending("x-content-type-options", "nosniff")
        .appending("x-frame-options", "DENY")
}
