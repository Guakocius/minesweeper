use xmltree::Element;
use std::fs::File;
use anyhow::{Result, bail};

pub struct Field {
    is_bomb: bool,
    is_opened: bool,
    is_flag: bool = false,
}

impl Field {
    fn to_xml() -> Element {
        let mut field = Element::new("field");
        field.children.push(XMLNode::Element({
            let mut e = Element::new("is-bomb");
            e.text = Some(self.is_bomb.to_string());
            e
        }));

        field.children.push(XMLNode::Element({
            let mut e = Element::new("is-opened");
            e.text = Some(self.is_opened.to_string());
            e
        }));

        field.children.push(XMLNode::Element({
            let mut e = Element::new("is-flag");
            e.text = Some(self.is_flag.to_string());
            e
        }));
        field
    }

    fn get_children(elem: &Element, name: &str) -> Result<bool> {
        let child = elem
            .get_child(name)
            .and_then(|c| c.text.as_deref())
            .ok_or_else(|| anyhow::anyhow!("missing <{}>", name))?;

        child.parse::<bool>()
            .map_err(|_| anyhow::anyhow!("invalid boolean in <{}>", name))
    }
    fn from_xml(elem: &Element) -> Result<Self> {
        Ok(Self {
            is_bomb: get_children(elem, "is-bomb")?,
            is_opened: get_children(elem, "is-opened")?,
            is_flag: get_children(elem, "is-flag")?,
        })
    }
}