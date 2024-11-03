use std::usize;

use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

fn normalise_color_value(color: &Color) -> (i32, i32, i32) {
    let red = (color.red * 255.0).round() as i32;
    let green = (color.green * 255.0).round() as i32;
    let blue = (color.blue * 255.0).round() as i32;
    (red.clamp(0, 255), green.clamp(0, 255), blue.clamp(0, 255))
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = y * self.width + x;
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        let index = y * self.width + x;
        &self.pixels[index]
    }

    pub fn to_ppm(&self) -> String {
        let mut result = format!("P3\n{} {}\n255\n", self.width, self.height);
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = &self.pixels[index];
                let values = normalise_color_value(pixel);
                line.push_str(&format!("{} {} {} ", values.0, values.1, values.2));
            }
            result.push_str(&line.trim_end());
            result.push_str("\n");
        }
        result
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

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red.clone());
        assert_eq!(*(c.pixel_at(2, 3)), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<_>>();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<_>>();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }
}
