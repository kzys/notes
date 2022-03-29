use handlebars::{to_json, Handlebars};
use itertools::Itertools;
use pulldown_cmark::{html, Event, HeadingLevel::H1, Parser, Tag};
use serde::Serialize;
use serde_json::value::{self, Map, Value as Json};
use std::error;
use std::fs;

#[derive(Serialize)]
struct Page {
    title: Option<String>,
    html: String,
    html_path: String,
    size: u64,
}

fn find_title<'a>(it: impl Iterator<Item = Event<'a>>) -> Option<String> {
    let mut heading = false;
    let mut title = None;
    it.for_each(|ev| match ev {
        Event::Start(Tag::Heading(H1, _, _)) => heading = true,
        Event::End(Tag::Heading(H1, _, _)) => heading = false,
        Event::Text(text) => {
            if heading {
                title = Some(text.to_string())
            }
        }
        _ => {}
    });
    title
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut reg = Handlebars::new();
    let tp = fs::read_to_string("system/template.html")?;
    reg.register_template_string("tp", tp)?;

    fs::create_dir_all("build")?;

    let paths = fs::read_dir(".")?;
    let mut pages = Vec::new();
    for path in paths {
        let path = path?;
        if !path.file_name().to_string_lossy().ends_with(".md") {
            continue;
        }

        let converted_path = path
            .file_name()
            .to_str()
            .ok_or("err")?
            .replace(".md", ".html");
        let html_path = if converted_path == "README.html" {
            "index.html"
        } else {
            &converted_path
        };

        let content = fs::read_to_string(path.path())?;
        let parser = Parser::new(&content);

        let (it1, it2) = parser.tee();
        let title = find_title(it1);

        let mut html = String::new();
        html::push_html(&mut html, it2);

        let size = fs::metadata(path.path())?.len();

        pages.push(Page {
            title,
            html,
            html_path: html_path.to_string(),
            size,
        });
    }

    let toc: Vec<&Page> = pages
        .iter()
        .sorted_by(|a, b| a.title.as_ref().unwrap().cmp(b.title.as_ref().unwrap()))
        .filter(|x| x.html_path != "index.html")
        .collect();

    for page in &pages {
        let mut data = Map::new();
        data.insert(
            "title".to_string(),
            to_json(page.title.as_ref().or(Some(&"Untitled".to_string()))),
        );
        data.insert("size".to_string(), to_json(page.size));
        data.insert("page".to_string(), to_json(&page));

        if page.html_path == "index.html" {
            data.insert("pages".to_string(), to_json(&toc));
        }

        let dest = format!("build/{}", page.html_path);
        let f = fs::File::create(dest)?;
        reg.render_to_write("tp", &data, f)?;
    }

    Ok(())
}
