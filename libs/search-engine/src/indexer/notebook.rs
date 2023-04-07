use std::{fs::read_dir, path::Path};

use persistance::fs::path_to_data_structure;
use wikitext::parsers::Note;

use crate::{tokenizer::tokenize, Doc};

use super::Proccessor;

#[derive(Default, Debug)]
pub(crate) struct Notebook {
    pub(crate) documents: Vec<Doc>,
}

impl Proccessor for Notebook {
    fn load(&mut self, location: &Path) {
        // For some reason using tokio::read_dir never returns in the while loop
        let entries = read_dir(location).unwrap();
        self.documents = entries
            .filter_map(|entry| {
                if let Ok(..) = entry {
                    let entry = entry.unwrap();
                    if let Some(fname) = entry.file_name().to_str() {
                        if fname.ends_with(".txt") {
                            let mut content = path_to_data_structure(&entry.path()).unwrap();
                            if content.header.get("title").is_none() {
                                let fixed_name = fname.strip_suffix(".txt").unwrap();
                                content.header.insert("title".into(), fixed_name.to_owned());
                            }

                            let doc = tokenize_note_meta(&content);
                            Some(doc)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Doc>>();
    }
}

pub(crate) fn tokenize_note_meta(content: &Note) -> Doc {
    let mut tokeniziable_content = content.content.clone();
    let tags = content.header.get("tags");
    let title = content.header.get("title");
    // create space between content and tags
    tokeniziable_content.push(' ');
    tokeniziable_content.push_str(tags.unwrap_or(&String::from("")));
    // create space between content and title
    tokeniziable_content.push(' ');
    tokeniziable_content.push_str(title.unwrap_or(&String::from("")));
    let mut tokenized_entry = tokenize(&tokeniziable_content);
    // TODO: Continue to fine tune weighting for different aspects of the note
    let title_tokens = tokenize(title.unwrap());
    for token in title_tokens.keys() {
        if let Some(title_token) = tokenized_entry.get_mut(token) {
            *title_token *= 3;
        } else {
            println!("Failed to tokenize {} in {:?}", token, tokenized_entry);
        }
    }

    Doc {
        id: title.unwrap().to_owned(),
        tokens: tokenized_entry,
        content: content.content.to_owned(),
    }
}
