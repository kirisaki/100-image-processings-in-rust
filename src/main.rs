use image::{Rgba, io::Reader as ImageReader};

fn main() {
    // q_001: Swap RGB channels
    {
        let mut img = ImageReader::open("dataset/images/imori_256x256.png")
            .unwrap().decode().unwrap().into_rgba8();
        for p in img.enumerate_pixels_mut() {
            *p.2 = Rgba([p.2[2], p.2[1], p.2[0], p.2[3]]);
        }
      let _ = img.save("test.png");
    }

    // q_002: Grayscale
    {
        let mut img = ImageReader::open("dataset/images/imori_256x256.png")
            .unwrap().decode().unwrap().into_rgba8();
        for p in img.enumerate_pixels_mut() {
            let g = ((0.2126 * (p.2[0] as f32 / 256.0) +
                        0.7152 * (p.2[1] as f32 / 256.0) +
                        0.0722 * (p.2[2] as f32 / 256.0)
                    ) * 256.0) as u8;
            *p.2 = Rgba([g, g, g, p.2[3]]);
        }
      let _ = img.save("test.png");
    }
}
