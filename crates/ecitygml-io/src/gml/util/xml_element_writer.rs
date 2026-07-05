use crate::Error;
use crate::gml::util::xml_element::XmlElement;
use crate::gml::write::Formatting;
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::se::Serializer;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNodeParts {
    pub attributes: Vec<(String, String)>,
    pub content: Vec<XmlNodeContent>,
}

impl XmlNodeParts {
    pub fn new(content: Vec<XmlNodeContent>) -> Self {
        Self {
            attributes: vec![],
            content,
        }
    }

    pub fn empty() -> Self {
        Self {
            attributes: vec![],
            content: vec![],
        }
    }

    pub fn write_contents_to_at_depth<W: Write>(
        &self,
        writer: &mut Writer<W>,
        formatting: Formatting,
        depth: usize,
    ) -> Result<(), quick_xml::Error> {
        for item in &self.content {
            match item {
                XmlNodeContent::Child(node) => node.write_to_at_depth(writer, formatting, depth)?,
                XmlNodeContent::Raw(raw) if !raw.is_empty() => {
                    write_raw(writer.get_mut(), raw, formatting, depth)?;
                }
                XmlNodeContent::Raw(_) => {}
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlNodeContent {
    Child(XmlNode),
    Raw(String),
}

impl XmlNodeContent {
    fn write_to<W: Write>(
        &self,
        writer: &mut Writer<W>,
        formatting: Formatting,
        depth: usize,
    ) -> Result<(), quick_xml::Error> {
        match self {
            Self::Child(node) => node.write_to_at_depth(writer, formatting, depth + 1),
            Self::Raw(raw) if !raw.is_empty() => {
                write_raw(writer.get_mut(), raw, formatting, depth + 1)
            }
            Self::Raw(_) => Ok(()),
        }
    }
}

impl From<XmlNode> for XmlNodeContent {
    fn from(node: XmlNode) -> Self {
        Self::Child(node)
    }
}

impl From<String> for XmlNodeContent {
    fn from(raw: String) -> Self {
        Self::Raw(raw)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNode {
    pub name: XmlElement,
    pub parts: XmlNodeParts,
}

impl XmlNode {
    pub fn new(name: XmlElement, parts: XmlNodeParts) -> Self {
        Self { name, parts }
    }

    pub fn write_to<W: Write>(
        &self,
        writer: &mut Writer<W>,
        formatting: Formatting,
    ) -> Result<(), quick_xml::Error> {
        self.write_to_at_depth(writer, formatting, 0)
    }

    pub fn to_string(&self, formatting: Formatting) -> Result<String, quick_xml::Error> {
        let mut buf = Vec::new();
        let mut writer = Writer::new(&mut buf);
        self.write_to(&mut writer, formatting)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    pub fn write_to_at_depth<W: Write>(
        &self,
        writer: &mut Writer<W>,
        formatting: Formatting,
        depth: usize,
    ) -> Result<(), quick_xml::Error> {
        write_prefix(writer.get_mut(), formatting, depth)?;

        let attributes: Vec<(&str, &str)> = self
            .parts
            .attributes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        writer.write_event(Event::Start(
            BytesStart::new(self.name.as_str()).with_attributes(attributes),
        ))?;

        for item in &self.parts.content {
            item.write_to(writer, formatting, depth)?;
        }
        if !self.parts.content.is_empty() {
            write_prefix(writer.get_mut(), formatting, depth)?;
        }

        writer.write_event(Event::End(BytesEnd::new(self.name.as_str())))?;

        Ok(())
    }
}

fn write_raw<W: Write>(
    writer: &mut W,
    raw: &str,
    formatting: Formatting,
    depth: usize,
) -> Result<(), quick_xml::Error> {
    match formatting {
        Formatting::Compact => {
            writer.write_all(raw.as_bytes())?;
        }
        Formatting::NewLine => {
            writer.write_all(b"\n")?;
            writer.write_all(raw.as_bytes())?;
        }
        Formatting::Indent { char, size } => {
            let indent: String = std::iter::repeat_n(char, depth * size).collect();
            for line in raw.lines() {
                writer.write_all(b"\n")?;
                writer.write_all(indent.as_bytes())?;
                // Strip serde's root-level indentation (one level = size chars),
                // preserving relative nesting beyond that.
                let stripped = line.get(size..).unwrap_or(line);
                writer.write_all(stripped.as_bytes())?;
            }
        }
    }
    Ok(())
}

fn write_prefix<W: Write>(
    writer: &mut W,
    formatting: Formatting,
    depth: usize,
) -> std::io::Result<()> {
    match formatting {
        Formatting::Compact => {}
        Formatting::NewLine => {
            writer.write_all(b"\n")?;
        }
        Formatting::Indent { char, size } => {
            writer.write_all(b"\n")?;
            let indent: String = std::iter::repeat_n(char, depth * size).collect();
            writer.write_all(indent.as_bytes())?;
        }
    }
    Ok(())
}

pub fn serialize_inner<T: serde::Serialize>(
    gml: T,
    formatting: Formatting,
) -> Result<Option<String>, Error> {
    const ROOT: &str = "_";
    let mut xml_body = String::new();
    let mut ser = Serializer::with_root(&mut xml_body, Some(ROOT))?;
    match formatting {
        Formatting::Compact => {}
        Formatting::NewLine => {
            ser.indent(' ', 0);
        }
        Formatting::Indent { char, size } => {
            ser.indent(char, size);
        }
    }
    gml.serialize(ser)?;

    // Strip the sentinel root wrapper dynamically to handle root tags with attributes
    // (e.g. <_ gml:id="...">content</_> instead of just <_>content</_>)
    let inner = xml_body
        .find('>')
        .map(|i| {
            let after = &xml_body[i + 1..];
            after.strip_prefix('\n').unwrap_or(after)
        })
        .and_then(|s| {
            s.rfind('<').map(|j| {
                let before = &s[..j];
                before.strip_suffix('\n').unwrap_or(before).to_string()
            })
        })
        .filter(|s| !s.trim().is_empty());

    Ok(inner)
}
