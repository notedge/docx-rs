use std::borrow::Cow;

use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:left")]
pub struct LeftMargin<'a> {
    #[xml(attr = "w:w")]
    pub size: Option<usize>,
    #[xml(attr = "w:type")]
    pub ty: Option<Cow<'a, str>>,
}

impl<'a> LeftMargin<'a> {
    __setter!(ty: Option<Cow<'a, str>>);
    __setter!(size: Option<usize>);
}

// __xml_test_suites!(
//     TopBorder,
//     TopBorder::default(),
//     r#"<w:top/>"#,
//     TopBorder::default().color("000000"),
//     r#"<w:top w:color="000000"/>"#,
//     TopBorder::default().shadow(false),
//     r#"<w:top w:shadow="false"/>"#,
//     TopBorder::default().space(40usize),
//     r#"<w:top w:space="40"/>"#,
//     TopBorder::default().size(20usize),
//     r#"<w:top w:sz="20"/>"#,
//     TopBorder::default().style(BorderStyle::Dotted),
//     r#"<w:top w:val="dotted"/>"#,
// );