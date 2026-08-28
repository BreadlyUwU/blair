use crate::config::{BLAIR, RunningMode};
use std::{fs, sync::LazyLock};
use fs_walk::{self, WalkOptions};
use chrono::prelude::*;
use markdown::{CompileOptions, Options};
use serde::Deserialize;
use rayon::prelude::*;

const PAGES_DIR: &str = "./content/pages";
const BLOG_DIR: &str = "./content/blog";

pub static PAGE_COLLECTION: LazyLock<Vec<Page>> = LazyLock::new(|| yooka_laylee(YLMode::Page));
pub static BLOG_COLLECTION: LazyLock<Vec<Page>> = LazyLock::new(|| yooka_laylee(YLMode::Blog));

#[allow(dead_code)]
#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub slug: Option<String>,
    pub updated: Option<String>,
    pub draft: Option<bool>,
    pub ap_link: Option<String>,
    pub at_link: Option<String>,
    pub tags: Option<Vec<String>>,
    pub extra: Option<Vec<(String, String)>>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Page {
    pub id: String, // <- Basically the slang, used as the endpoint: either the '.md' file folder's name, or the frontmatter's slang if present
    pub date: Option<NaiveDate>, // Requied for blog posts, not for regular pages
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
    let mut collection: Vec<Page> = Vec::new();

    // Collectionite go brrr
    let walker_root = match mode {
        YLMode::Page => {
            // Si B⋅LAIR est démarré en mode Debug, alors une page de test sera générée et accessible à l'endpoint /test-page
            if let RunningMode::Debug = &BLAIR.running_mode { collection.push(create_dummy_page()) }
            // Et let's get back to business
            WalkOptions::new()
                            .max_depth(2)
                            .ends_with(".md")
                            .walk(PAGES_DIR)
        }
        YLMode::Blog => WalkOptions::new()
                            .max_depth(2)
                            .ends_with(".md")
                            .walk(BLOG_DIR)
    };

    // Parse le fichier markdown trouvé pour le dégraisser comme un niveau de DK64
    let parsed_pages: Vec<Page> = walker_root
        .flatten()
        .par_bridge()
        .map(|entry| {
            log::debug!("Reading page: {:?}", entry);

            // « Lui sucer les yeux… »
            let file: String = fs::read_to_string(&entry).expect("The markdown file cannot be read. Why? Good luck finding out why.");

            // « Le retourner comme un gant… »
            let (frontmatter, markdown_body) = retrieve_frontmatter(&file);
            let markdown_content = markdown::to_html_with_options(markdown_body.as_str(), &Options {
                compile: CompileOptions {
                    allow_dangerous_html: true,
                    ..CompileOptions::default()
                },
                ..Options::default()
            });

            let stringified_entry_name = entry
                .parent()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .unwrap()
                .to_string();

            // Récupère rapidos la date directement du nom du dossier d'origine de l'entrée, si présente
            let post_date: Option<NaiveDate> = if stringified_entry_name.contains("_") {
                Some(stringified_entry_name.split("_").nth(0).unwrap_or_default().parse().unwrap_or_default())
            } else { None };

            // « Arracher les ongles… »
            let mut page = Page {
                id: stringified_entry_name,
                date: post_date,
                frontmatter: frontmatter.unwrap_or_default(),
                content: markdown_content.expect("How can you fuck up that much a markdown file like you just did, explain to me please, it's surreal")
            };

            // Voilà la partie du programme qui sera la plus probablement inutile
            if let Some(slug) = &page.frontmatter.slug {
                page.id = slug.clone();
            }

            // « Laissez-le courir, viande meilleure avec du sel ! »
            log::debug!("Parsing of \"{}\" completed, sending to the collection", &page.id);
            page
        }).collect();

    collection.extend(parsed_pages);

    // Si yooka_laylee roule pour le blog, à la fin du parsing trie les entrées par date
    if let YLMode::Blog = mode {
        collection.sort_by(|left, right| right.date.cmp(&left.date));
    }

    return collection
}

fn retrieve_frontmatter(source: &str) -> (Option<Frontmatter>, String) {
    let trim = source.trim_start();
    if !trim.starts_with("---") { return (None, source.to_string()); }

    let mut lines = trim.lines();
    lines.next();

    let mut fmt_lines = Vec::new();
    for line in lines.by_ref() {
        if line.trim() == "---" { break; }
        fmt_lines.push(line);
    }

    let body = lines.collect::<Vec<_>>().join("\n");

    match yaml_serde::from_str::<Frontmatter>(&fmt_lines.join("\n")) {
        Ok(frontmatter) => (Some(frontmatter), body),
        Err(e) => {
            log::error!("Error while parsing frontmatter: {}", e);
            (None, source.to_string())
        }
    }
}

pub fn is_potential_opt_file(uri: &str) -> bool { uri.contains(".") && !uri.ends_with(".md") }

// For debug:
#[allow(dead_code)]
fn create_dummy_page() -> Page {
    return Page {
        id: String::from("test-page"),
        date: Some("2069-01-01".parse().unwrap_or_default()),
        frontmatter: Frontmatter {
            title: String::from("This is a test"),
            description: String::from("This is only a test"),
            updated: Some(String::from("2026-08-16")),
            draft: Some(true),
            tags: Some(vec!["test".to_string(), "oh_yeah".to_string()]),
            extra: Some(vec![("Is this a test?".to_string(), "*HL1 Announcer Voice* Yes".to_string())]),
            ..Default::default()
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
