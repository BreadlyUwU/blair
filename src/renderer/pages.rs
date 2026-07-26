use crate::config::RunningMode;
use std::fs;
use fs_walk::{self, WalkOptions};
use markdown;

const PAGES_DIR: &str = "./content/pages";
const BLOG_DIR: &str = "./content/blog";

#[allow(dead_code)]
#[derive(Clone)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub slang: Option<String>,
    pub date: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub extra: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Page {
    pub frontmatter: Frontmatter,
    pub directory: Option<std::path::PathBuf>,
    pub filename: Option<String>, // Needed only if it isn't about the `index.md` file
    pub static_files: Option<Vec<std::path::PathBuf>>, // List of every optional files located at the page/article root folder
    pub content: String // <- The HTML output of the markdown parser
}

impl Page {
    pub fn slang(&self) -> Option<&str> {
        self.frontmatter.slang.as_deref()
    }
}

/**
========================= 📃
= Collecting the PAGIES = 📃
========================= 📃
Si un dossier à l'intérieur de "./content/{pages,blog}" contient
un fichier nommé "index.md" ALORS ce dernier sera parsé et la fonction
créera une struct `Page` qui sera ensuite réutilisé par Askama pour le rendu final

À explorer également plus tard:
 *  Faire en sorte que si dans un dossier il y a plusieurs fichiers MD,
   alors B⋅LAIR devra être capable de les servirs séparément:
   ex:
     /quelque-chose/index.md <- https://breadcat.run/quelque-chose
     /quelque-chose/more.md  <- https://breadcat.run/quelque-chose/more
   (/quelque-chose/more devra avoir également accès aux mêmes fichiers 
   statiques que /quelque-chose, mais c'est peut-être quelque-chose qu'Askama 
   pourra m'aider à gérer avec la fonction get_url(), à la condition que le 
   parsing des fichiers Markdown ne part pas en vrille à la présence de syntaxe Jinja)
**/
pub fn yooka_laylee() -> Vec<Page> {
    let mut page_collection: Vec<Page> = vec![];
    let mut page_collector: Vec<String>;
    if let RunningMode::Debug = crate::config::init().running_mode { 
        let dummy_page: Page = create_dummy_page();
        page_collection.push(dummy_page) 
    }
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

    return page_collection
}

// Parsing every blog posts present inside /content/blog-posts
// ↑ (May be also handled by yooka_laylee after all)
// fn banjo_kazooie() -> Vec<BlogPost> {}

// For debug:
#[allow(dead_code)]
fn create_dummy_page() -> Page {
    return Page {
        frontmatter: Frontmatter {
            title: String::from("This is a test"), 
            description: String::from("This is only a test"), 
            slang: Some(String::from("test-page")), 
            date: Some(20690101),
            tags: None,
            extra: None,
        },
        directory: None,
        filename: None,
        static_files: None,
        content: String::from("<h1>This is a test</h1>
<h2>This is only a test</h2>
<h3>Yet another test</h3>
<h4>I'm not in a hurry…</h4>
<h5>Yeah one more time baby</h5>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim aeque doleamus animo, cum corpore dolemus, fieri tamen permagna accessio potest, si aliquod aeternum et infinitum impendere malum nobis opinemur. Quod idem licet transferre in voluptatem, ut postea variari voluptas distinguique.</p>
"),
    }
}
