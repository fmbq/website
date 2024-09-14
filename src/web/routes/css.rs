use poem::{get, handler, IntoEndpoint, Route};

/// Generate an endpoint that compiles a SCSS file and returns it as CSS.
///
/// For debug builds, the compilation is done at runtime. For release builds,
/// the compilation is done once at compile time and the CSS is baked into the
/// binary.
macro_rules! scss_endpoint {
    ($path:expr) => {{
        use poem::IntoResponse;

        #[cfg(debug_assertions)]
        #[handler]
        fn compile() -> impl IntoResponse {
            grass::from_path(
                concat!(env!("CARGO_MANIFEST_DIR"), "/", $path),
                &grass::Options::default(),
            )
            .unwrap()
            .with_content_type("text/css")
        }

        #[cfg(not(debug_assertions))]
        #[handler]
        fn compile() -> impl IntoResponse {
            grass::include!($path).with_content_type("text/css")
        }

        compile
    }};
}

pub fn routes() -> impl IntoEndpoint {
    Route::new()
        .at("site.css", get(scss_endpoint!("src/web/scss/index.scss")))
        .at("login.css", get(scss_endpoint!("src/web/scss/login.scss")))
        .at("admin.css", get(scss_endpoint!("src/web/scss/admin.scss")))
}
