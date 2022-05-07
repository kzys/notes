use chrono::prelude::*;
use handlebars::{to_json, Handlebars};
use itertools::Itertools;
use pulldown_cmark::{html, Parser};
use serde::Serialize;
use serde_json::value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

mod excerpt;
mod hack;

#[derive(Serialize)]
struct Page {
    title: String,
    first_paragraph: String,
    html: String,
    html_path: String,
    size: u64,
    changes: Vec<u64>,
    created_at: Option<String>,
    last_modified_at: Option<String>,
}

impl Page {
    fn created_at(&self) -> Option<DateTime<Utc>> {
        if self.changes.len() == 0 {
            None
        } else {
            Some(Utc.timestamp(self.changes[0] as i64, 0))
        }
    }
    fn last_modified_at(&self) -> Option<DateTime<Utc>> {
        if self.changes.len() == 0 {
            None
        } else {
            let last = self.changes.len() - 1;
            Some(Utc.timestamp(self.changes[last] as i64, 0))
        }
    }
}

fn find_files(from: &Path, ext: &str) -> hack::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    let paths = fs::read_dir(from)?;
    for path in paths {
        let path = path?;
        let mut pb = PathBuf::from(&path.file_name());
        if from.as_os_str() != "." {
            pb = from.join(pb)
        }
        if let Ok(t) = path.file_type() {
            if t.is_dir() {
                result.append(&mut find_files(&pb, ext)?)
            }

            if let Some(e) = pb.extension() {
                if e == ext {
                    result.push(pb);
                }
            }
        }
    }
    Ok(result)
}

fn collect_pages(
    dir: &str,
    files: &std::collections::HashMap<String, Vec<u64>>,
) -> hack::Result<Vec<Page>> {
    let dir = Path::new(dir);
    let paths = find_files(dir, "md")?;
    let mut pages = Vec::new();
    for path in paths {
        let mut pb = path.clone();
        pb.set_extension("html");

        let html_path = if dir.as_os_str() != "." {
            PathBuf::from(pb)
        } else {
            dir.join(pb)
        };

        let content = fs::read_to_string(&path)?;
        let parser = Parser::new(&content);

        let (it1, it2) = parser.tee();
        let excerpt = excerpt::find_excerpt(it1);

        let mut html = String::new();
        html::push_html(&mut html, it2);

        let size = fs::metadata(&path)?.len();
        let empty = vec![];
        let k = path.to_str().unwrap().to_string();
        let changes = files.get(&k).unwrap_or(&empty);

        let untitled = "???".to_string();
        let title = excerpt.title.unwrap_or(untitled.clone());
        let first_paragraph = excerpt.first_paragraph;
        let mut p = Page {
            title: title.to_string(),
            first_paragraph: first_paragraph.to_string(),
            html,
            html_path: html_path.to_string_lossy().to_string(),
            size,
            changes: changes.to_vec(),
            created_at: None,
            last_modified_at: None,
        };
        p.created_at = p.created_at().map(|x| x.format("%Y/%m/%d").to_string());
        p.last_modified_at = p
            .last_modified_at()
            .map(|x| x.format("%Y/%m/%d").to_string());
        pages.push(p);
    }
    Ok(pages)
}

fn git_log() -> hack::Result<std::collections::HashMap<String, Vec<u64>>> {
    let args = vec![
        "log",
        "--format=format:XX\t%H\t%ct",
        "--name-status",
        "--reverse",
    ];
    let git_log = Command::new("git").args(args).output();
    let stdout_vec = git_log?.stdout;
    let stdout = std::str::from_utf8(&stdout_vec)?;
    let lines = stdout.split("\n");

    let mut files = std::collections::HashMap::<String, Vec<u64>>::new();

    let mut dt: Option<u64> = None;
    for line in lines {
        let columns: Vec<&str> = line.split("\t").collect();
        if columns.len() > 2 && columns[0] == "XX" {
            dt = Some(u64::from_str(columns[2])?);
        } else if columns.len() == 2 {
            let name = columns[1].to_string();
            files.entry(name).or_insert(vec![]).push(dt.unwrap());
        } else if columns.len() == 3 {
            let from = columns[1].to_string();
            let to = columns[2].to_string();
            let past_commits = files.get(&from).unwrap_or(&vec![]).to_vec();
            files.insert(to, past_commits);
        } else {
            dt = None;
        }
    }

    Ok(files)
}

fn main() -> hack::Result<()> {
    let css_name = "main.css";

    let mut reg = Handlebars::new();
    let tp = fs::read_to_string("system/template.html")?;
    reg.register_template_string("tp", tp)?;

    let build_dir = Path::new("build");

    fs::create_dir_all(build_dir)?;
    fs::copy("system/main.css", build_dir.join(css_name))?;

    let files = git_log()?;
    let pages: Vec<Page> = collect_pages(".", &files)?;

    let toc: Vec<&Page> = pages
        .iter()
        .sorted_by(|a, b| a.title.cmp(&b.title))
        .filter(|x| x.html_path != "index.html")
        .collect();

    for page in &pages {
        let mut data = value::Map::new();
        data.insert("title".to_string(), to_json(&page.title));
        data.insert("size".to_string(), to_json(page.size));
        data.insert("page".to_string(), to_json(&page));

        if page.html_path.ends_with("index.html") {
            data.insert("pages".to_string(), to_json(&toc));
        }

        let mut css_path = "".to_string();
        let dest = build_dir.join(&page.html_path);
        if let Some(p) = dest.parent() {
            fs::create_dir_all(p)?;
            css_path += &"../".repeat(p.components().count() - 1);
        }
        css_path += "main.css";
        data.insert("css_path".to_string(), to_json(css_path));

        let f = fs::File::create(dest)?;
        reg.render_to_write("tp", &data, f)?;
    }

    Ok(())
}
