use image::{Rgba, io::Reader as ImageReader};

fn main() {
    // q_001: Swap RGB channels
    {
        let mut img = ImageReader::open("dataset/images/imori_256x256.png")
            .unwrap().decode().unwrap().into_rgba8();
        for p in img.enumerate_pixels_mut() {
            *p.2 = Rgba([p.2[2], p.2[1], p.2[0], p.2[3]]);
        }
      let _ = img.save("results/q_001_swap_rgb.png");
    }

    // q_002: Grayscale
    {
        let mut img = ImageReader::open("dataset/images/imori_256x256.png")
            .unwrap().decode().unwrap().into_rgba8();
        for p in img.enumerate_pixels_mut() {
            let q = ((0.2126 * (p.2[0] as f32 / 256.0) +
                        0.7152 * (p.2[1] as f32 / 256.0) +
                        0.0722 * (p.2[2] as f32 / 256.0)
                    ) * 256.0) as u8;
            *p.2 = Rgba([q, q, q, p.2[3]]);
        }
      let _ = img.save("results/q_002_grayscale.png");
    }

    // q_003: Binary
    {
        let mut img = ImageReader::open("dataset/images/imori_256x256.png")
            .unwrap().decode().unwrap().into_rgba8();
        for p in img.enumerate_pixels_mut() {
            let q = ((0.2126 * (p.2[0] as f32 / 256.0) +
                        0.7152 * (p.2[1] as f32 / 256.0) +
                        0.0722 * (p.2[2] as f32 / 256.0)
                    ) * 256.0) as u8;
            let r = if q > 127 {255}else{0};
            *p.2 = Rgba([r, r, r, p.2[3]]);
        }
      let _ = img.save("results/q_003_binary.png");

    }
}
