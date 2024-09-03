//! PDF generation for a rulebook.

use crate::domain::rules::Rulebook;
use color_eyre::eyre::Result;
use printpdf::{PdfDocument, Pt};
use std::io::{BufWriter, Write};

pub fn generate_rulebook_pdf(_rulebook: &Rulebook, destination: impl Write) -> Result<()> {
    let page_width = Pt(72.0 * 8.5);
    let page_height = Pt(72.0 * 11.0);

    let (doc, _page1, _layer1) = PdfDocument::new(
        "PDF_Document_title",
        page_width.into(),
        page_height.into(),
        "Layer 1",
    );

    doc.save(&mut BufWriter::new(destination))?;

    Ok(())
}
