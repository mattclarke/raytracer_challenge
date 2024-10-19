use std::usize;

use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let zero_pixel = Color::new(0.0, 0.0, 0.0);
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels.iter() {
            assert_eq!(*pixel, zero_pixel);
        }
    }
}
