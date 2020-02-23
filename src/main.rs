extern crate image;
extern crate num_complex;

fn main() {
    const IMG_X: u32 = 5000;
    const IMG_Y: u32 = 5000;

    let scalex = 3.0 / IMG_X as f32;
    let scaley = 3.0 / IMG_Y as f32;

    let mut imgbuf = image::ImageBuffer::new(IMG_X, IMG_Y);

    for x in 0..IMG_X {
        for y in 0..IMG_Y {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;

            *pixel = image::Rgb([data[0], data[2], i as u8]); // blue
        }
    }

    imgbuf.save("output/image.png").unwrap();
}