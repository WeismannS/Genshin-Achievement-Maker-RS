extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::fs::File;
use std::path::Path;
use rusttype::{Font, Scale};
use image::{Rgba, ImageBuffer, EncodableLayout, ImageResult};
use imageproc::drawing::draw_text;
use std::include_bytes;
use std::io;
use std::io::{BufWriter, Write};
use std::os::windows::prelude::FileExt;
use imageproc::definitions::Image;

fn achievement(mut cx: FunctionContext) -> Image<Rgba<u8>>  {
    let s = cx.argument::<JsString>(0)?.value(&mut cx);
    if s.len() > 70 { panic!("String is too long") };
    let red: Rgba<u8> = Rgba([140, 125, 111, 250]);
    let og_length = s.len();
    let sliced = if og_length > 35 { &s[..35] } else { &s };
    let space = sliced.rfind(char::is_whitespace).unwrap_or(0);
    let chooped_right = if og_length > 35 { format!("{}-", &s[..space]) } else { s.to_string() };
    let chooped_left = if og_length > 35 { &s[space..] } else { "" };
    let path = Path::new("./src/img.png");
    let mut img = image::open(path).unwrap();
    let font_data = include_bytes!("font.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    let mut image2 = draw_text(&mut img, red, 220, 120, Scale::uniform(28.0), &font, &chooped_right);
    let image3 = draw_text(&mut image2, red, 220, 155, Scale::uniform(28.0), &font, chooped_left);
 image3
}

fn main() {

}