use pulldown_cmark::{html, Event, HeadingLevel::H1, Parser, Tag};

pub struct Excerpt {
    pub title: Option<String>,
    pub first_paragraph: String,
}

enum State {
    Unknown,
    InHeading,
    InFirstParagraph,
}

pub fn find_excerpt<'a>(it: impl Iterator<Item = Event<'a>>) -> Excerpt {
    let mut s = State::Unknown;
    let mut title = None;
    let mut first_paragraph = String::new();

    for ev in it {
        match ev {
            Event::Start(Tag::Heading(H1, _, _)) => s = State::InHeading,
            Event::End(Tag::Heading(H1, _, _)) => s = State::Unknown,
            Event::Start(Tag::Paragraph) => s = State::InFirstParagraph,
            Event::End(Tag::Link(_, _, _)) => (),
            Event::End(_) => break,
            Event::Text(text) => match s {
                State::InHeading => title = Some(text.to_string()),
                State::InFirstParagraph => first_paragraph += &text,
                _ => (),
            },
            _ => {}
        }
    }
    Excerpt {
        title,
        first_paragraph,
    }
}
