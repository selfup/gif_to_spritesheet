extern crate image;

use core::panic;
use image::gif::GifDecoder;
use image::{AnimationDecoder, GenericImageView};
use image::{Frame, Frames};
use std::fs::File;

fn main() {
    let img = image::open("player.gif").unwrap();

    let gif_file = match File::open("player.gif") {
        Ok(file) => file,
        Err(_err) => panic!(),
    };

    let decoder: GifDecoder<File> = GifDecoder::new(gif_file).unwrap();
    let frames: Frames = decoder.into_frames();
    let frames: Vec<Frame> = frames.collect_frames().expect("error decoding gif");

    let height: u32 = img.height();
    let width: u32 = img.width();
    let mut rgba_images: Vec<u8> = vec![];
    let mut rgba_images_frames: u32 = 0;

    for frame in frames.into_iter() {
        let frame_buffer = frame.into_buffer();

        for buf in frame_buffer.into_raw() {
            rgba_images.push(buf);
        }

        rgba_images_frames += 1;
    }

    let final_width = width * rgba_images_frames as u32;

    image::save_buffer(
        "player.png",
        &rgba_images,
        height,
        final_width,
        image::ColorType::Rgba8,
    )
    .unwrap()
}
