use crate::config::RunningMode;
use fs_walk::{self, WalkOptions};
use std::fs;
use markdown;

const PAGES_DIR: &str = "./content/pages";
const BLOG_DIR: &str = "./content/blog";

#[allow(dead_code)]
#[derive(Clone)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub slang: Option<String>,
    pub draft: Option<bool>,
    pub date: Option<u32>,
    pub tags: Option<Vec<String>>,
    pub extra: Option<Vec<(String, String)>>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Page {
    pub id: String, // <- Basically the slang, used as the endpoint: either the '.md' file folder's name, or the frontmatter's slang if present
    pub frontmatter: Frontmatter,
    pub content: String // <- The HTML output of the markdown parser
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

    // Si B⋅LAIR est démarré en mode Debug, alors une page de test sera générée et accessible à l'endpoint /test-page
    if let RunningMode::Debug = crate::config::init().running_mode { page_collection.push(create_dummy_page()) }

    let walker_root = WalkOptions::new()
        .max_depth(2)
        .ends_with(".md")
        .walk(PAGES_DIR);

    for entry in walker_root.flatten() {
        log::debug!("Reading page: {:?}", entry);
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
        id: String::from("test-page"),
        frontmatter: Frontmatter {
            title: String::from("This is a test"), 
            description: String::from("This is only a test"), 
            slang: None, 
            draft: Some(false),
            date: Some(20690101),
            tags: None,
            extra: None,
        },
        content: String::from("<img src='./assets/dreamcat.jpg' style='max-width: 30vw; display: block; float: right;'/>
<center><audio controls src='./assets/this_is_a_test.opus' autoplay></audio></center>
<h1>This is a test</h1>
<h2>This is only a test</h2>
<h3>Yet another test</h3>
<h4>I'm not in a hurry…</h4>
<h5>Yeah one more time baby</h5>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim aeque doleamus animo, cum corpore dolemus, fieri tamen permagna accessio potest, si aliquod aeternum et infinitum impendere malum nobis opinemur. Quod idem licet transferre in voluptatem, ut postea variari voluptas distinguique.</p>
"),
    }
}

pub fn is_potential_opt_file(uri: &str) -> bool {
    uri.ends_with(".png") || uri.ends_with(".jpg") || uri.ends_with(".gif") || uri.ends_with(".webp") || uri.ends_with(".opus")
}