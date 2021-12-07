
use std::{iter::{repeat}, rc::Rc};

use image::{Rgba, io::Reader as ImageReader, ImageBuffer, SubImage, png, GenericImage, GenericImageView, DynamicImage, imageops::crop};

#[derive(Debug, Clone)]
struct Hsv {
    h: f32,
    s: f32,
    v: f32,
}

fn main() {
    let imori256 = "dataset/images/imori_256x256.png";

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

    // q_005: HSV conversion
    apply(imori256, "results/q_005_hsv_conversion.png", |img| {
        // RGB to HSV
        let mut hsv: Vec<Vec<Hsv>> = repeat(repeat(Hsv{h: 0.0, s: 0.0, v: 0.0})
            .take(img.height() as usize)
            .collect())
            .take(img.width() as usize)
            .collect();
        for p in img.enumerate_pixels_mut() {
            let v_max = [p.2[0], p.2[1], p.2[2]].iter().max().unwrap().clone();
            let v_min = [p.2[0], p.2[1], p.2[2]].iter().min().unwrap().clone();
            let s = v_max as f32 - v_min as f32;
            let h = match v_min {
                _ if v_min == v_max => 0.0,
                _ if v_min == p.2[2] => 60.0 * (p.2[1] as f32 - p.2[0] as f32) / s + 60.0,
                _ if v_min == p.2[0] => 60.0 * (p.2[2] as f32 - p.2[1] as f32) / s + 180.0,
                _ => 60.0 * (p.2[0] as f32 - p.2[2] as f32) / s + 300.0,
            };
            let v = v_max as f32;
            hsv[p.0 as usize][p.1 as usize] = Hsv{h, s, v};
        }
        // Change hue
        for row in hsv.iter_mut() {
            for p in row.iter_mut() {
                p.h = (p.h + 180.0) % 360.0;
                p.s = p.s;
                p.v = p.v;
            }
        }

        // HSV to RGB
        for (x, row) in hsv.iter().enumerate() {
            for (y, p) in row.iter().enumerate() {
                let hp = p.h / 60.0;
                let xp = p.s as f32 * (1.0 - (hp % 2.0 - 1.0).abs()); 
                let m = p.v - p.s;
                let (r, g, b) = match hp {
                    _ if hp < 1.0              => (p.s, xp, 0.0),
                    _ if 1.0 <= hp && hp < 2.0 => (xp, p.s, 0.0), 
                    _ if 2.0 <= hp && hp < 3.0 => (0.0, p.s, xp), 
                    _ if 3.0 <= hp && hp < 4.0 => (0.0, xp, p.s), 
                    _ if 4.0 <= hp && hp < 5.0 => (xp, 0.0, p.s), 
                    _ if 5.0 <= hp && hp < 6.0 => (p.s, 0.0, xp),
                    _ => panic!("invalid hue")
                };
                let q = Rgba([
                    (r + m).floor() as u8,
                    (g + m).floor() as u8,
                    (b + m).floor() as u8,
                    img.get_pixel(x as u32, y as u32)[3],
                ]);
                img.put_pixel(x as u32, y as u32, q);
            }
        }
    });

    // q_006: Color subtraction
    apply(imori256, "results/q_006_color_subtraction.png", |img| {
        let th = (256_u32 / 4_u32) as u8;
        let f = |q| q / th * th + th / 2; 
        for p in img.pixels_mut() {
            *p = Rgba([f(p[0]), f(p[1]), f(p[2]), p[3]]);
        }
    });

    // q_007: Average pooling
    apply(imori256, "results/q_007_average_pooling.png", |img| {
        let k = 5;
        let (w, h) = (img.width(), img.height());
        let (n_x, n_y) = (w / k, h / k);
        let mut buf = DynamicImage::new_rgba8(k * (n_x + 1), k * (n_y + 1)).to_rgba8();
        let _ = buf.copy_from(img, 0, 0);
        let buf_ref = buf.clone();

        // Fill mergin
        for p in buf.enumerate_pixels_mut() {
            match (p.0, p.1) {
                (x, y) if x < w && y < h => (),
                (x, y) if w <= x && y < h => *p.2 = *buf_ref.get_pixel(w - 1, y),
                (x, y) if x < w && h <= y => *p.2 = *buf_ref.get_pixel(x, h - 1),
                _ => *p.2 = *buf_ref.get_pixel(w - 1, h - 1),
            }
        }

        // Mean pixels
        let mut mean: Vec<Vec<[u8; 4]>> = repeat(repeat([0, 0, 0, 0])
            .take(n_x as usize + 1)
            .collect())
            .take(n_y as usize + 1)
            .collect();
        for x_k in 0..n_x {
            for y_k in 0..n_y {
                mean[x_k as usize][y_k as usize] = buf
                    .sub_image(x_k * k, y_k * k, k, k)
                    .pixels()
                    .fold([0.0, 0.0, 0.0, 0.0],
                        |a, (_, _, x)| [a[0] + x[0] as f32, a[1] + x[1] as f32, a[2] + x[2] as f32, a[3] + x[3] as f32]).map(|x| (x / (k * k) as f32) as u8);
            }
        }
        
        // Fill chunks by mean
        for p in buf.enumerate_pixels_mut(){
            let (x, y) = (p.0, p.1);
            let (x_k, y_k) = (x / k, y / k);
            *p.2 = Rgba(mean[x_k as usize][y_k as usize]);
        }
        buf = crop(&mut buf, 0, 0, w, h).to_image();
        let _ = img.copy_from(&buf, 0, 0);
        
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
