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
const BLOG_DIR: &str = "./content/blog";

struct Frontmatter {
    title: String,
    description: String,
    slang: Option<String>,
    date: Option<String>,
    tags: Option<Vec<String>>,
    extra: Option<Vec<String>>,
}

pub struct Page {
    front: Frontmatter,
    directory: std::path::PathBuf,
    // filename: String, // Must be at the root of the page's directory
    static_files: Option<Vec<std::path::PathBuf>>,
}

/**
========================= 📃
= Collecting the PAGIES = 📃
========================= 📃
Si un dossier à l'intérieur de "./content/{pages,blog}" contient
un fichier nommé "index.md" ALORS ce dernier sera parsé et la fonction
inscrira dans un Vec:
 *  Le nom de la page/titre d'article
 *  La description
 *  Le nom du dossier dans lequel il se trouve (qui servira de slang);
 *  (Si) la date de l'article (normalement récupérable dans le nom du dossier)
 *  Les tags de l'article ou (peu probablement) de la page
 *  Et enfin le HTML du contenu parsé à partir du markdown

À explorer également plus tard:
 *  Faire en sorte que si dans un dossier il y a plusieurs fichiers MD,
   alors B⋅LAIR devra être capable de les servirs séparément:
   ex:
     /quelque-chose/index.md <- https://breadcat.run/quelque-chose
     /quelque-chose/more.md  <- https://breadcat.run/quelque-chose/more
   (/quelque-chose/more devra avoir également accès aux mêmes fichiers statiques que /quelque-chose, mais c'est peut-être quelque-chose qu'Askama pourra m'aider à gérer avec la fonction get_url(), à la condition que le parsing des fichiers Markdown ne part pas en vrille à la présence de syntaxe Jinja)
**/
fn yooka_laylee() /* -> Vec<Page> */ {
    let mut page_collection: Vec<(u32, Page)>;
    let mut page_collector: Vec<String>;
    let walker_root = WalkOptions::new()
        .max_depth(2)
        .ends_with(".md")
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
// ↑ (May be also handled by yooka_laylee after all)
// fn banjo_kazooie() -> Vec<BlogPost> {}

// For debug:
pub async fn test_page(){
 //
}
