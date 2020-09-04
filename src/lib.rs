extern crate js_sys;
extern crate web_sys;

use std::io::{Cursor,SeekFrom,Seek,Read};
use wasm_bindgen::prelude::*;
use image::{self,DynamicImage,ImageFormat};
use base64::{encode};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> Result<(), JsValue> {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    let base64_str = get_image_as_base64(img);
    return append_img(base64_str);
}

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(error) => {
            panic!("{:?}", error)
        }
    };
    return img;
}

fn get_image_as_base64(_img: DynamicImage) -> String {
    // 创建一个内存空间
    let mut c = Cursor::new(Vec::new());
    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    // 从内存读取数据
    c.read_to_end(&mut out).unwrap();
    // 解码
    let stt = encode(&mut out);
    let together = format!("{}{}", "data:image/png;base64,", stt);
    return together;
}


pub fn append_img(image_src: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("img")?;
    val.set_attribute("src", &image_src)?;
    val.set_attribute("style", "height: 200px")?;
    body.append_child(&val)?;
    Ok(())
}
