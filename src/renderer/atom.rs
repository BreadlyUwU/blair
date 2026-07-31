use std::sync::LazyLock;
use crate::config::BLAIR;
use crate::renderer::pages::BLOG_COLLECTION;
use actix_web::{HttpResponse, Responder, get};
use atom_syndication::{Category, Content, Entry, Feed, Link, Person, Text};

static FEED: LazyLock<String> = LazyLock::new(|| build());

#[get("/atom.xml")]
pub async fn feed() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/atom+xml; charset=utf-8")
        .body(FEED.as_str())
}

fn build() -> String {
    let base_url = &BLAIR.current_base_url;
    let blog_base_url = format!("{}/blog/", &base_url);
    let mut atom_feed = Vec::new();
    let mut atom = Feed::default();
    let mut latest_updated: Option<atom_syndication::FixedDateTime> = None;
    let mut entries = Vec::new();

    atom.set_title("Journal of the Breadcat");
    atom.set_id(&blog_base_url);
    atom.set_icon(format!("{}/favicon.png", &base_url));
    atom.set_authors(vec![Person {
        name: "Aleenn".to_string(),
        ..Default::default()
    }]);
    atom.set_links(vec![
        Link {
            href: format!("{}/atom.xml", &BLAIR.current_base_url),
            rel: "self".to_string(),
            mime_type: Some("application/atom+xml".to_string()),
            ..Default::default()
        },
        Link {
            href: blog_base_url.clone(),
            rel: "alternate".to_string(),
            mime_type: Some("text/html".to_string()),
            ..Default::default()
        },
    ]);

    for post in BLOG_COLLECTION.iter().filter(|post| {
        post.frontmatter.draft != Some(true)
    }) {
        let published_dt = match &post.date {
            Some(date) => {
                let timestamp = format!("{}T12:00:00+00:00", date);
                chrono::DateTime::parse_from_rfc3339(&timestamp).unwrap()
            }
            None => chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00+00:00").unwrap(),
        };

        let entry_url = format!("{}/{}/", &blog_base_url.trim_end_matches("/"), &post.id);
        let entry = Entry {
            title: Text::plain(&post.frontmatter.title),
            id: entry_url.clone(),
            updated: published_dt,
            authors: vec![Person {
                name: "Breadcat's Lair".to_string(),
                ..Default::default()
            }],
            categories: post.frontmatter.tags.as_ref().map_or_else(Vec::new, |tags| {
                tags.iter().map(|tag| Category {
                    term: tag.clone(),
                    ..Default::default()
                }).collect()
            }),
            contributors: Vec::new(),
            links: vec![Link {
                href: entry_url,
                rel: "alternate".to_string(),
                mime_type: Some("text/html".to_string()),
                ..Default::default()
            }],
            published: Some(published_dt),
            rights: None,
            source: None,
            summary: Some(Text::plain(post.frontmatter.description.clone())),
            content: Some(Content {
                value: Some(post.content.clone()),
                content_type: Some("html".to_string()),
                ..Default::default()
            }),
            extensions: Default::default(),
            namespaces: Default::default(),
        };

        latest_updated = match latest_updated {
            Some(current) if current > published_dt => Some(current),
            _ => Some(published_dt),
        };
        entries.push(entry);
    }

    atom.set_updated(latest_updated.unwrap_or_else(|| {
        chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00+00:00").unwrap()
    }));
    atom.set_entries(entries);

    atom.write_to(&mut atom_feed).unwrap();
    log::info!("Atom feed initialized");
    String::from_utf8(atom_feed).unwrap()
}