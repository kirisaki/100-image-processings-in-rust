use image::{GenericImage, Rgba, io::Reader as ImageReader};

fn main() {
    let mut img = ImageReader::open("dataset/images/imori_256x256.png")
        .unwrap().decode().unwrap().into_rgba8();
    for p in img.enumerate_pixels_mut() {
        *p.2 = Rgba([255, 140, 200, 160]);
    }
    img.save("test.png");
}
