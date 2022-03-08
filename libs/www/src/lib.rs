use ::build::{config::General, RefHubTx};

#[cfg(not(debug_assertions))]
use ::build::get_data_dir_location;

use render::GlobalBacklinks;
use std::{path::PathBuf, sync::Arc};
use tasks::normalize_wiki_location;
use warp::Filter;

pub mod controllers;
pub mod handlers;
pub mod services;

use crate::handlers::*;

pub(crate) type RefHubParts = (GlobalBacklinks, RefHubTx);

pub async fn server(config: General, parts: RefHubParts) {
    let wiki_location = Arc::new(config.wiki_location);
    let media_location = Arc::new(normalize_wiki_location(&config.media_location));
    let static_page_router = StaticPageRouter {
        user: Arc::new(config.user),
        media_location: media_location.clone(),
    };
    let wiki_router = WikiPageRouter {
        parts,
        wiki_location: wiki_location.clone(),
    };

    let task_router = TaskPageRouter::new(wiki_location.clone());
    let static_files_router = StaticFileRouter {
        media_location: media_location.clone(),
    };
    let api_router = APIRouter {
        wiki_location,
        media_location,
    };
    pretty_env_logger::init();
    // Order matters!!
    let log = warp::log("toplevel");
    let routes = warp::any()
        .and(
            static_files_router
                .routes()
                .or(static_page_router.routes())
                .or(api_router.routes())
                .or(task_router.routes())
                .or(wiki_router.routes())
                .or(static_page_router.index())
                .recover(handle_rejection),
        )
        .with(log);
    let port: u16 = config.port;
    println!("┌──────────────────────────────────────────────┐");
    println!("│Starting web backend @ http://127.0.0.1:{}  │", port);
    println!("└──────────────────────────────────────────────┘");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

#[cfg(debug_assertions)]
fn get_static_dir() -> PathBuf {
    PathBuf::from("static")
}

#[cfg(not(debug_assertions))]
fn get_static_dir() -> PathBuf {
    let data_dir = get_data_dir_location();
    data_dir.join("static")
}
