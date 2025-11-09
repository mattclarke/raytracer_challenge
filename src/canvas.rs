use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

fn normalise_color_value(color: &Color) -> [i32; 3] {
    let red = (color.red * 255.0).round() as i32;
    let green = (color.green * 255.0).round() as i32;
    let blue = (color.blue * 255.0).round() as i32;
    [red.clamp(0, 255), green.clamp(0, 255), blue.clamp(0, 255)]
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
        // PPM format:
        // first line is version: always "P3"
        // second line is "width space height"
        // third line is the max colour value: 255
        // Next lines are the pixels in RGB order
        //
        // PPM line limit is 70 chars including the newline, so needs to be wrapped
        let mut result = format!("P3\n{} {}\n255\n", self.width, self.height);
        for y in 0..self.height {
            let mut line = String::with_capacity(70);
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = &self.pixels[index];
                let values = normalise_color_value(pixel);

                if line.is_empty() {
                    line.push_str(&format!("{} {} {}", values[0], values[1], values[2]));
                    continue;
                }
                // Room for all values
                let full = &format!(" {} {} {}", values[0], values[1], values[2]);
                if line.len() + full.len() <= 69 {
                    line.push_str(full);
                    continue;
                }
                // Line filling up, so add one at a time
                for v in values.iter() {
                    let sv = format!(" {}", v);
                    if line.len() + sv.len() >= 69 {
                        result.push_str(&line);
                        result.push('\n');
                        line = String::with_capacity(70);
                    }
                    line.push_str(&sv);
                }
            }
            result.push_str(&line);
            result.push('\n');
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

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<_>>();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            " 153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            " 153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_files_terminated_by_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
