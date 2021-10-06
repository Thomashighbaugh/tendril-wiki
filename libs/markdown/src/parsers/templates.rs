use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File, ReadDir},
    io::Read,
    sync::{Arc, Mutex},
};

use tasks::search::SearchResult;

use sailfish::TemplateOnce;

use crate::parsers::format_links;

pub struct BasicPage<'a> {
    title: &'a String,
    body: &'a String,
    tags: &'a Vec<String>,
    raw_md: &'a str,
    metadata: &'a HashMap<String, String>,
    backlinks: Vec<String>,
    render_static: bool,
}

#[derive(TemplateOnce)]
#[template(path = "user_style.stpl")]
pub struct StylesPage {
    pub body: String,
}

#[derive(TemplateOnce)]
#[template(path = "file_list.stpl")]
pub struct UploadedFilesPage {
    pub entries: ReadDir,
}

#[derive(TemplateOnce)]
#[template(path = "new_page.stpl")]
pub struct NewPage<'a> {
    pub title: Option<String>,
    pub linkto: Option<&'a String>,
    pub action_params: Option<&'a str>,
}

#[derive(TemplateOnce)]
#[template(path = "login_page.stpl")]
pub struct LoginPage {}

#[derive(TemplateOnce)]
#[template(path = "help_page.stpl")]
pub struct HelpPage {}

#[derive(TemplateOnce)]
#[template(path = "search_page.stpl")]
pub struct SearchPage {}

#[derive(TemplateOnce)]
#[template(path = "search_results.stpl")]
pub struct SearchResultsPage {
    pub pages: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "file_uploader.stpl")]
pub struct FileUploader {}

#[derive(TemplateOnce)]
#[template(path = "search_results_context.stpl")]
pub struct SearchResultsContextPage {
    pub pages: Vec<SearchResult>,
}

#[derive(TemplateOnce)]
#[template(path = "tag_idx.stpl")]
pub struct TagIndex {
    pub tags: BTreeMap<String, Vec<String>>,
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct IndexPage {
    pub user: String,
}

#[derive(TemplateOnce)]
#[template(path = "tags.stpl")]
pub struct TagPage {
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "backlinks.stpl")]
pub struct LinkPage {
    pub links: BTreeMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct TemplattedPage {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub raw_md: String,
    pub metadata: HashMap<String, String>,
}

pub struct ParsedTemplate {
    pub outlinks: Vec<String>,
    pub page: TemplattedPage,
}

pub type TagMapping = Arc<Mutex<BTreeMap<String, Vec<String>>>>;
pub type GlobalBacklinks = Arc<Mutex<BTreeMap<String, Vec<String>>>>;
pub type ParsedPages = Arc<Mutex<Vec<TemplattedPage>>>;

pub fn render_template(
    page: &TemplattedPage,
    links: Option<&Vec<String>>,
    render_static: bool,
) -> String {
    let mut backlinks = match links {
        Some(links) => links.to_owned(),
        None => Vec::new(),
    };
    backlinks.dedup();
    let tag_string = page
        .tags
        .iter()
        .map(|t| format!("<li><a href=\"/tags/{}\">#{}</a></li>", t, t))
        .collect::<Vec<String>>()
        .join("\n");
    let backlinks_string = backlinks
        .iter()
        .map(|l| format!("<a href=\"{}\">{}</a>", format_links(l), l))
        .collect::<Vec<String>>()
        .join("\n");
    let mut ctx = File::open("templates/main.html").unwrap();
    let mut ctx_string = String::new();
    ctx.read_to_string(&mut ctx_string).unwrap();
    ctx_string = ctx_string
        .replace("<%= title %>", &page.title)
        .replace("<%= body %>", &page.body)
        .replace("<%= tags %>", &tag_string)
        .replace("<%= links %>", &backlinks_string);
    let parsed = ctx_string.split('\n');
    parsed
        .map(|line| {
            if line.trim().starts_with("<%= include") {
                parse_includes(line.trim())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")

    // let ctx = BasicPage {
    //     title: &page.title,
    //     tags: &page.tags,
    //     body: &page.body,
    //     metadata: &page.metadata,
    //     raw_md: &page.raw_md,
    //     backlinks,
    //     render_static,
    // };
    //ctx.render_once().unwrap()
}

fn parse_includes(include_str: &str) -> String {
    println!("{}", include_str);
    let included_file = include_str
        .strip_prefix("<%= include \"")
        .unwrap()
        .strip_suffix("\" %>")
        .unwrap();
    let template = format!("templates/{}.html", included_file);
    println!("{}", template);
    fs::read_to_string(template).unwrap()
}

pub fn write_index_page(user: String) {
    let ctx = IndexPage { user };
    fs::write("public/index.html", ctx.render_once().unwrap()).unwrap();
}

pub fn write_entries(pages: &ParsedPages, backlinks: &GlobalBacklinks) {
    let page_vals = pages.lock().unwrap();
    let link_vals = backlinks.lock().unwrap();
    for page in page_vals.iter() {
        let links = link_vals.get(&page.title);
        let output = render_template(&page, links, true);
        // TODO use path here instead of title? Since `/` in title can cause issues in fs::write
        fs::create_dir(format!("public/{}", page.title.replace('/', "-"))).unwrap();
        fs::write(
            format!("public/{}/index.html", page.title.replace('/', "-")),
            output,
        )
        .unwrap();
    }
}

pub fn write_tag_pages(map: TagMapping, pages: &ParsedPages) {
    let tag_map = map.lock().unwrap();
    for key in tag_map.keys() {
        let title = key.to_string();
        let tags = tag_map.get(key).unwrap().to_owned();
        let pages = pages.lock().unwrap();
        let page = pages.iter().find(|pg| pg.title == title);
        if let Some(template) = page {
            let output = render_template(template, Some(&tags), true);
            fs::create_dir(format!("public/tags/{}", title)).unwrap();
            fs::write(format!("public/tags/{}/index.html", title), output).unwrap();
        } else {
            let ctx = TagPage {
                title: title.clone(),
                tags,
            };
            fs::create_dir(format!("public/tags/{}", title)).unwrap();
            fs::write(
                format!("public/tags/{}/index.html", title),
                ctx.render_once().unwrap(),
            )
            .unwrap();
        }
    }
}

pub fn write_tag_index(map: TagMapping) {
    let tag_map = map.lock().unwrap();
    let ctx = TagIndex {
        tags: tag_map.clone(),
    };
    fs::write("public/tags/index.html", ctx.render_once().unwrap()).unwrap();
}

pub fn write_backlinks(map: GlobalBacklinks) {
    let link_map = map.lock().unwrap();
    let ctx = LinkPage {
        links: link_map.clone(),
    };
    fs::write(
        "public/links/index.html".to_string(),
        ctx.render_once().unwrap(),
    )
    .unwrap();
}
