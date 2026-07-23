use crate::config;
use super::functions;
use super::functions::BlairStandardFunctions;
use std::fs;
use fs_walk::{self, WalkOptions};
use markdown;
use actix_web::{
    HttpResponse, 
    Responder, 
    get
};
use askama::Template;

const PAGES_DIR: &str = "./content/pages";
// const BLOG_DIR: &str = "./content/blog";

struct Frontmatter {
    title: String,
    description: Option<String>,
    date: Option<String>,
    tags: Option<Vec<String>>,
    slang: String,
}

struct Page {
    front: Frontmatter,
    directory: String, // idk yet if "directory" should point to the FS location or the app endpoint
    filename: String, // Must be at the root of the page's directory
    opt_files: Option<Vec<String>>,
}

struct BlogPost {
    front: Frontmatter,
    directory: String,
    opt_files: Option<Vec<String>>,
}

// Collecting the PAGIES 📃📃📃
fn yooka_laylee() /* -> Vec<Page> */ {
    let mut page_collection: Vec<(u32, Page)>;
    let mut page_collector: Vec<String>;
    let walker_root = WalkOptions::new()
        // .dirs()
        .max_depth(2)
        .walk(PAGES_DIR);
    
    // for dir in walker.flatten().enumerate() {
    //     let mut collector: Vec<std::path::PathBuf> = Vec::new();

    //     let index_file = WalkOptions::new().files().extension("md").walk(&dir.1);
    //     let index_file_flatten = index_file.flatten();
    //     for root_file in index_file_flatten {
    //         collector.push(root_file);
    //     }
    //     for dir_inside_dir in WalkOptions::new().dirs().walk(&dir.1).flatten().enumerate() {
    //         let file_inside_dir = WalkOptions::new().files().walk(&dir_inside_dir.1);
    //     }
    // }

    for entry in walker_root.flatten() {
        println!("{:?}", entry);
    }
}

// Parsing every blog posts present inside /content/blog-posts
// fn banjo_kazooie() -> Vec<BlogPost> {}

// For debug:
pub async fn test_page(){
 //
}