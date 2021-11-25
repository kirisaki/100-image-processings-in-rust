use image::{flat::Error, io::Reader as ImageReader};

fn main() {
    let img = ImageReader::open("dataset/images/imori_256x256.png").unwrap().decode().unwrap();
    img.save("test.png");
}
