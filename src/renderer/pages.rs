use crate::config::BLAIR;
use crate::config::RunningMode;
use fs_walk::{self, WalkOptions};
use markdown::{CompileOptions, Options};
use serde::Deserialize;
use std::fs;


const PAGES_DIR: &str = "./content/pages";
const BLOG_DIR: &str = "./content/blog";

#[allow(dead_code)]
#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub slug: Option<String>,
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
pub enum YLMode { Page, Blog }
pub fn yooka_laylee(mode: YLMode) -> Vec<Page> {
    let mut collection: Vec<Page> = vec![];

    // Si B⋅LAIR est démarré en mode Debug, alors une page de test sera générée et accessible à l'endpoint (blog)/test-page
    if let RunningMode::Debug = &BLAIR.running_mode { collection.push(create_dummy_page()) }

    // Collectionite go brrr
    let walker_root = match mode {
        YLMode::Page => WalkOptions::new()
                            .max_depth(2)
                            .ends_with(".md")
                            .walk(PAGES_DIR),
        YLMode::Blog => WalkOptions::new()
                            .max_depth(2)
                            .ends_with(".md")
                            .walk(BLOG_DIR)
    };

    // Parse le fichier markdown trouvé pour le dégraisser comme un niveau de DK64
    for entry in walker_root.flatten() {
        log::debug!("Reading page: {:?}", entry);

        // « Lui sucer les yeux… »
        let file: String = fs::read_to_string(&entry).unwrap();
        
        // « Le retourner comme un gant… »
        let (frontmatter, markdown_body) = retrieve_frontmatter(&file);
        let markdown_content = markdown::to_html_with_options(markdown_body.as_str(), &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
            ..Options::default()
        });

        // « Arracher les ongles… »
        let mut page = Page {
            id: entry
                .parent()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .unwrap()
                .to_string(),
            frontmatter: frontmatter.unwrap_or_default(),
            content: markdown_content.unwrap()
        };

        // Voilà la partie du programme qui sera la plus probablement inutile
        if let Some(slug) = &page.frontmatter.slug {
            page.id = slug.clone();
        }

        // « Laissez-le courir, viande meilleure avec du sel ! »
        log::debug!("Parsing of \"{}\" completed, sending to the collection", &page.id);
        collection.push(page)
    };

    return collection
}

fn retrieve_frontmatter(source: &str) -> (Option<Frontmatter>, String) {
    let trim = source.trim_start();
    if !trim.starts_with("---") {
        return (None, trim.to_string());
    }

    let mut lines = trim.lines();
    let _opening_delimiter = lines.next();

    let mut fmt_lines = Vec::new();
    for line in lines.by_ref() {
        if line.trim() == "---" { break; }
        fmt_lines.push(line);
    }
    
    let body = lines.collect::<Vec<_>>().join("\n");
    let fmt_text = fmt_lines.join("\n");

    match yaml_serde::from_str::<Frontmatter>(&fmt_text) {
        Ok(frontmatter) => (Some(frontmatter), body),
        Err(e) => {
            log::warn!("Error while parsing frontmatter: {}", e);
            (None, source.to_string())
        }
    }
}

pub fn is_potential_opt_file(uri: &str) -> bool {
    uri.ends_with(".png") || uri.ends_with(".jpg") || uri.ends_with(".gif") || uri.ends_with(".webp") || uri.ends_with(".opus")
}

// For debug:
#[allow(dead_code)]
fn create_dummy_page() -> Page {
    return Page {
        id: String::from("test-page"),
        frontmatter: Frontmatter {
            title: String::from("This is a test"), 
            description: String::from("This is only a test"), 
            slug: None, 
            draft: Some(true),
            date: Some(20690101),
            tags: Some(vec!["test".to_string()]),
            extra: Some(vec![("Is this a test?".to_string(), "*HL1 Announcer Voice* Yes.".to_string())]),
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
