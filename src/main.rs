
use image::{Rgba, io::Reader as ImageReader, ImageBuffer};

fn main() {
    let imori256 = "dataset/images/madara_256x256.png";

    // q_001: Swap RGB channels
    apply(imori256, "results/q_001_swap_rgb.png", |img|{
        for p in img.enumerate_pixels_mut() {
            *p.2 = Rgba([p.2[2], p.2[1], p.2[0], p.2[3]]);
        }
    });

    // q_002: Grayscal
    apply(imori256, "results/q_002_grayscale.png", |img|{
        for p in img.enumerate_pixels_mut() {
            let q = ((0.2126 * (p.2[0] as f32 / 256.0) +
                        0.7152 * (p.2[1] as f32 / 256.0) +
                        0.0722 * (p.2[2] as f32 / 256.0)
                    ) * 256.0) as u8;
            *p.2 = Rgba([q, q, q, p.2[3]]);
        }
    });

    // q_003: Binary
    apply(imori256, "results/q_003_binary.png", |img|{
        for p in img.enumerate_pixels_mut() {
            let q = ((0.2126 * (p.2[0] as f32 / 256.0) +
                        0.7152 * (p.2[1] as f32 / 256.0) +
                        0.0722 * (p.2[2] as f32 / 256.0)
                    ) * 256.0) as u8;
            let r = if q > 127 {255}else{0};
            *p.2 = Rgba([r, r, r, p.2[3]]);
        }
    });

    // q_004: Otsu's binarization
    apply(imori256, "results/q_004_otsu_binary.png", |img| {
        let mut hist: [u32; 256] = [0; 256];
        for p in img.enumerate_pixels_mut() {
            let q = ((0.2126 * (p.2[0] as f32 / 255.0) +
                        0.7152 * (p.2[1] as f32 / 255.0) +
                        0.0722 * (p.2[2] as f32 / 255.0)
                    ) * 255.0) as u8;
            hist[q as usize] += 1;
            *p.2 = Rgba([q, q, q, p.2[3]]);
        }
        let th = (1_u32..255).filter_map(|t| {
            let (h1, h2) = hist.split_at(t as usize);
            let n1 = h1.iter().sum::<u32>() as f32;
            let n2 = h2.iter().sum::<u32>() as f32;
            let m1 = h1.iter().enumerate().fold(0, |a, (k, v)| a + k as u32 * v) as f32 / n1;
            let m2 = h2.iter().enumerate().fold(0, |a, (k, v)| a + (255 - k) as u32 * v) as f32 / n2;
            let u = (t, n1 * n2 * (m1 - m2).powi(2));
            if  u.1.is_normal() {
                Some(u)
            } else {
                None
            }
        }).fold((0, 0.0),
            |(k0, v0), (k1, v1)| (if v1 > v0 {k1} else {k0}, v1.max(v0)))
          .0 as u8;
        println!("{:?}", th);
        for p in img.enumerate_pixels_mut() {
            if p.2[0] < th {
                *p.2 = Rgba([0, 0, 0, p.2[3]])
            } else {
                *p.2 = Rgba([255, 255, 255, p.2[3]])
            };
        }
    });
}

fn apply<F>(input: &str, outoput: &str, mut f: F)
where
    F: FnMut(&mut ImageBuffer<Rgba<u8>, Vec<u8>>)
{
    let mut img = ImageReader::open(input).unwrap().decode().unwrap().into_rgba8();
    f(&mut img);
    let _ = img.save(outoput);
}
