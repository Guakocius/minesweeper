use xmltree::{XMLNode, Element};
use std::fs::File;
use anyhow::{Result, bail, anyhow};

#[derive(Debug, Clone)]
pub struct Field {
    pub is_bomb: bool,
    pub is_opened: bool,
    pub is_flag: bool,
}

impl Field {
    pub fn to_xml(&self) -> Element {
        let mut field = Element::new("field");
        field.children.push(XMLNode::Element({
            let mut e = Element::new("is-bomb");
            e.children.push(XMLNode::Text(self.is_bomb.to_string()));
            e
        }));

        field.children.push(XMLNode::Element({
            let mut e = Element::new("is-opened");
            e.children.push(XMLNode::Text(self.is_opened.to_string()));
            e
        }));

        field.children.push(XMLNode::Element({
            let mut e = Element::new("is-flag");
            e.children.push(XMLNode::Text(self.is_flag.to_string()));
            e
        }));
        field
    }

    pub fn get_children(elem: &Element, name: &str) -> Result<bool> {
        let child = elem
            .get_child(name)
            .ok_or_else(|| anyhow!("missing <{}>", name))?;
        let text = child
            .get_text()
            .ok_or_else(|| anyhow!("missing text in <{}>", name))?;

        text.parse::<bool>()
            .map_err(|_| anyhow!("invalid boolean in <{}>", name))
    }
    pub fn from_xml(elem: &Element) -> Result<Self> {
        Ok(Self {
            is_bomb: Field::get_children(elem, "is-bomb")?,
            is_opened: Field::get_children(elem, "is-opened")?,
            is_flag: Field::get_children(elem, "is-flag")?,
        })
    }
}