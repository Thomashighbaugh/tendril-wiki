use tasks::CompileState;

use crate::{get_template_file, render_includes, Render};

pub struct UploadedFilesPage {
    pub entries: Vec<String>,
}

impl UploadedFilesPage {
    pub fn new(entries: Vec<String>) -> Self {
        Self { entries }
    }
    fn render_entries(&self) -> String {
        let mut entry_list = String::new();
        for entry in &self.entries {
            entry_list.push_str(&format!("<a href=\"/files/{}\">{}</a>", entry, entry));
        }
        entry_list
    }
}

impl Render for UploadedFilesPage {
    fn render(&self, state: &CompileState) -> String {
        let mut ctx = get_template_file("file_list").unwrap();
        ctx = ctx.replace("<%= entries %>", &self.render_entries());
        render_includes(ctx, state, None)
    }
}