extern crate docx_rs;

use docx_rs::{Docx, Justification, StyleExt};

fn main() {
  let path = std::path::Path::new("hello_world.docx");
  let file = std::fs::File::create(&path).unwrap();

  let mut docx = Docx::new();

  docx
    .create_style()
    .with_name("TestStyle")
    .with_sz(42)
    .with_color("ff0000");

  docx
    .create_para()
    .add_text("hello, world")
    .with_style_name("TestStyle")
    .with_jc(&Justification::Start);

  docx
    .create_para()
    .add_text("hello, world")
    .with_style_name("TestStyle")
    .with_jc(&Justification::Center);

  docx
    .create_para()
    .add_text("hello, world")
    .with_style_name("TestStyle")
    .with_jc(&Justification::End);

  docx.generate(file).unwrap();
}