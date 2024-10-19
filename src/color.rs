struct Color {
    red: f32,
    blue: f32,
    green: f32
}

fn color(red: f32, green: f32, blue: f32) -> Color {
    Color {
        red,
        green,
        blue,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = color(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }
}
