use handlebars::{to_json, Handlebars};
use itertools::Itertools;
use pulldown_cmark::{html, Event, HeadingLevel::H1, Parser, Tag};
use serde_json::value::{self, Map, Value as Json};
use std::error;
use std::fs;

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

        let mut html_output = String::new();
        html::push_html(&mut html_output, it2);
        let mut data = Map::new();
        data.insert(
            "title".to_string(),
            to_json(title.or(Some("Untitled".to_string()))),
        );
        data.insert("main".to_string(), to_json(html_output));

        let dest = format!("build/{}", html_path);
        let f = fs::File::create(dest)?;
        reg.render_to_write("tp", &data, f)?;
    }

    Ok(())
}
