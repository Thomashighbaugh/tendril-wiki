use crate::parsers::{to_html, NoteHeader, ParsedPages, ParsedTemplate, TemplattedPage};

pub mod tags;

use self::tags::*;

pub fn to_template(note: &NoteHeader) -> ParsedTemplate {
    let html = to_html(&note.content);
    let default_title = "Untitled".to_string();
    let title = note
        .metadata
        .get("title")
        .unwrap_or(&default_title)
        .to_owned();
    let tags = match note.metadata.get("tags") {
        None => Vec::with_capacity(0),
        Some(raw_tags) => TagsArray::new(raw_tags).values,
    };
    let mut rendered_metadata = note.metadata.to_owned();
    // We're already showing this, so no need to dump it in the table...
    rendered_metadata.remove("title");
    rendered_metadata.remove("tags");
    let desc = if note.content.len() >= 100 {
        let mut shortened_desc = note.content.clone();
        shortened_desc.truncate(80);
        shortened_desc.push_str("...");
        shortened_desc
    } else {
        note.content.clone()
    };
    let page = TemplattedPage {
        title,
        tags,
        body: html.body,
        metadata: rendered_metadata,
        desc,
    };
    ParsedTemplate {
        outlinks: html.outlinks,
        page,
    }
}

pub async fn update_templatted_pages(page: TemplattedPage, pages: ParsedPages) {
    let mut tempatted_pages = pages.lock().await;
    tempatted_pages.push(page);
}