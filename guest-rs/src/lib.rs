wit_bindgen::generate!({
    world: "component",
});

use std::io::Cursor;

use exports::intf::Guest;
use exports::intf::GuestImage;
use exports::intf::Image;
use exports::intf::ImageBorrow;
use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageFormat;
use once_cell::sync::Lazy;
use regex::Regex;

struct Component;

export!(Component);

impl GuestImage for DynamicImage {}

impl Guest for Component {
    type Image = DynamicImage;

    fn print(input: String) -> () {
        println!("{}", input);
    }

    fn hello() -> String {
        "Hello from Rust!".to_owned()
    }

    fn load_image(bytes: Vec<u8>) -> Image {
        Image::new(image::load_from_memory(&bytes).unwrap())
    }

    fn resize_image(img: ImageBorrow<'_>, width: u32, height: u32) -> Image {
        let img = img
            .get::<DynamicImage>()
            .resize(width, height, FilterType::Lanczos3);
        Image::new(img)
    }

    fn image_to_bytes(this: ImageBorrow<'_>) -> Vec<u8> {
        let mut buf = Vec::new();
        this.get::<DynamicImage>()
            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
            .unwrap();
        buf
    }

    fn extract_emails(input: String) -> Vec<String> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"[A-Za-z0-9-_.]+@[A-Za-z0-9-_.]+\.[A-Za-z0-9]{2,4}").unwrap());

        REGEX
            .find_iter(&input)
            .map(|m| m.as_str().to_owned())
            .collect()
    }
}

#[test]
fn test_extract_emails() {
    let input = "john.doe@gmail.com and jane.doe@gmail.com".to_owned();
    let expected = vec![
        "john.doe@gmail.com".to_owned(),
        "jane.doe@gmail.com".to_owned(),
    ];
    assert_eq!(Component::extract_emails(input), expected);
}
