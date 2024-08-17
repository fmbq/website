use maud::Markup;
use poem::{handler, web::Html, Response};

#[handler]
pub fn get_html() -> Html<Markup> {
    Html(crate::pages::rules::render())
}

/// Download the current rulebook as a PDF.
#[handler]
pub fn get_pdf() -> Response {
    let rulebook = crate::domain::rules::get_rulebook();
    let mut pdf_bytes = Vec::new();
    crate::services::pdf_rules::generate_rulebook_pdf(rulebook, &mut pdf_bytes).unwrap();

    let filename = format!("rulebook-{}.pdf", rulebook.effective);

    Response::builder()
        .content_type("application/pdf")
        .header("Content-Disposition", format!("inline; filename={filename}"))
        .body(pdf_bytes)
}
