# B⋅LAIR (Breadcat's LAIR) 😺🍞

## What's dis

B⋅LAIR is a custom hybrid static site generator (powered by [Askama](https://www.askama.rs/)) and web server (powered by [Actix](https://actix.rs/)).<br/>
It's the software that serve my website.

## How it works

When starting (and more precisely the first time anyone accesse the website after B⋅LAIR started), it will parse each folders/files inside the content folder and generates pages accordingly:

``` md
content/
|-- pages/
    |-- blair/
        |-- index.md    (-> breadcat.run/blair)
        |-- a-l-aise.md (-> breadcat.run/blair/a-l-aise)
        |-- breadkitty.gif
        |-- screenshot_neocities.webp
```
Every optional files located inside `blair` folder will be available at `breadcat.run/blair/`.<br/>
Also, any additionnal markdown files that isn't named "index" will make B⋅LAIR generate a sub-page.

```
|-- content/
    |-- blog/
        |-- 2026-07-30_hello-world /
            |-- _thumbnail.gif 
            |-- index.md (-> breadcat.run/blog/hello-world)
```
Same as above except everything parsed inside the `blog` folder will be loaded inside a different collection. It makes the generation of the atom feed easier.<br/>
Also, in the blog's particular case, the format of the folder's name will be parsed as such:<br/>
`2026-07-30 <_> hello-world` <- Everything before "_" is the date, everything after will be the post's ID, also used for the post's url. This is where the post's date is written and not inside the post's frontmatter.

Speaking of the frontmatter, every markdown files have a YAML-based frontmatter at the top of it. Exemple:

```yaml
---
title: Hello World Final Mix
description: You'll (finaly) see the creation of new lives
tags:
 - misc
---
```

And here's the struct associated with it:

```rust
pub struct Frontmatter {
    pub title: String,
    pub description: String,
    pub slug: Option<String>, // ← Here for a historical reason but currently unused
    pub updated: Option<String>, // Must be a (YYYY-MM-DD date), tho also currently unused
    pub draft: Option<bool>, // ← Drafted blog posts will be visible only when B⋅LAIR is running in Debug Mode (aka not built/run with cargo's `--release` flag).
    pub ap_link: Option<String>, // ← Currently unused (for a (maybe) future feature)
    pub at_link: Option<String>, // ← Currently unused (for a (maybe) future feature)
    pub tags: Option<Vec<String>>,
    pub extra: Option<Vec<(String, String)>>,
}
```

Struct that is part of this one:

```rust
pub struct Page {
    pub id: String, // <- Basically the slang, used as the endpoint: either the '.md' file folder's name, the frontmatter's slang if present, or directly the file-name if this one isn't called "index".
    pub root: String, // The parent folder name, automatically parsed
    pub date: Option<NaiveDate>, // Requied only for blog posts
    pub frontmatter: Frontmatter, // <- Hey hey hi hi :3
    pub content: String // <- The markdown parser's HTML output
}
```

Everything inside the `Page` struct is accessible from the Askama template: <br/>
```jinja
{{ page.frontmatter.title }}
```
🤓👆<(_`page` is ackthually `posts` (which is an array of `post`) inside the `/blog` template and `post` for an individual blog post_)

> Inside the `./content` folder there's also the self-describing `static` folder.<br/>
> In my case the `./content` folder is a git submodule located inside a separate local Forgejo instance, this is why you cannot access it. The description above is all you need to know if you want to use B⋅LAIR for yourself.

## Why

> « Because sometimes you just need to rewrite everything yourself. » - Rust developers apparently.

Because I ditched Zola as my static site generator when it started to heavily rely on LLM-written code, I didn't want to wait and see how long it would take for the project to really start to stink.

With LLM you cannot script how the shit goes down. So it was an excellent excuse to start a project to help me learning programmation, and it was really worth it.

Learning and having fun doing it is the point of this project, nothing more and nothing less.

## A little warning if you plan to fork B⋅LAIR for yourself

There's no **proper** configuration handler yet, the site's URL and listening IP/Port are currently hardcoded inside `config.rs`.<br/>