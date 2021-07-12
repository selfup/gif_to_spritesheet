use image::gif::GifDecoder;
use image::{AnimationDecoder, GenericImageView};
use image::{Frame, Frames};
use std::fs::File;

#[derive(Debug)]
pub struct G2SError(String);

pub fn convert(input: &str, output: &str) -> Result<(), G2SError> {
    let img = image::open(input).unwrap();

    let gif_file: File = match File::open(input) {
        Ok(file) => file,
        Err(err) => return Err(G2SError(err.to_string())),
    };

    let decoder: GifDecoder<File> = match GifDecoder::new(gif_file) {
        Ok(decoded) => decoded,
        Err(err) => return Err(G2SError(err.to_string())),
    };

    let frames: Frames = decoder.into_frames();

    let frames: Vec<Frame> = match frames.collect_frames() {
        Ok(frames) => frames,
        Err(err) => return Err(G2SError(err.to_string())),
    };

    let height_as_width: u32 = img.height();
    let width_as_height: u32 = img.width();

    let mut rgba_images: Vec<u8> = vec![];
    let mut rgba_images_frames: u32 = 0;

    for frame in frames.into_iter() {
        let frame_buffer = frame.into_buffer();

        for buf in frame_buffer.into_raw() {
            rgba_images.push(buf);
        }

        rgba_images_frames += 1;
    }

    let final_width = width_as_height * rgba_images_frames as u32;

    match image::save_buffer(
        output,
        &rgba_images,
        height_as_width,
        final_width,
        image::ColorType::Rgba8,
    ) {
        Ok(res) => return Ok(res),
        Err(err) => return Err(G2SError(err.to_string())),
    }
}

#[test]
fn it_converts_a_gif_into_a_spritesheet() -> Result<(), G2SError> {
    convert("player.gif", "player.png")
}
