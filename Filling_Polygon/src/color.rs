#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_hex(hex: u32) -> Self {
        let red = ((hex >> 16) & 0xFF) as u8;
        let green = ((hex >> 8) & 0xFF) as u8;
        let blue = (hex & 0xFF) as u8;
        Self { red, green, blue }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    pub fn display(&self) {
        println!("Color - R: {}, G: {}, B: {}", self.red, self.green, self.blue);
    }

    pub fn add(&self, other: Color) -> Self {
        Self {
            red: self.red.saturating_add(other.red),
            green: self.green.saturating_add(other.green),
            blue: self.blue.saturating_add(other.blue),
        }
    }

    pub fn multiply(&self, factor: f32) -> Self {
        Self {
            red: (self.red as f32 * factor).clamp(0.0, 255.0) as u8,
            green: (self.green as f32 * factor).clamp(0.0, 255.0) as u8,
            blue: (self.blue as f32 * factor).clamp(0.0, 255.0) as u8,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Color(R: {}, G: {}, B: {})", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_new() {
        let color = Color::new(255, 0, 0);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex(0xFF0000);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 0, 0);
        assert_eq!(color.to_hex(), 0xFF0000);
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(100, 100, 100);
        let color2 = Color::new(100, 100, 100);
        let result = color1.add(color2);
        assert_eq!(result.red, 200);
        assert_eq!(result.green, 200);
        assert_eq!(result.blue, 200);
    }

    #[test]
    fn test_add_overflow() {
        let color1 = Color::new(200, 200, 200);
        let color2 = Color::new(100, 100, 100);
        let result = color1.add(color2);
        assert_eq!(result.red, 255);
        assert_eq!(result.green, 255);
        assert_eq!(result.blue, 255);
    }

    #[test]
    fn test_multiply() {
        let color = Color::new(100, 100, 100);
        let result = color.multiply(2.0);
        assert_eq!(result.red, 200);
        assert_eq!(result.green, 200);
        assert_eq!(result.blue, 200);
    }

    #[test]
    fn test_multiply_overflow() {
        let color = Color::new(200, 200, 200);
        let result = color.multiply(2.0);
        assert_eq!(result.red, 255);
        assert_eq!(result.green, 255);
        assert_eq!(result.blue, 255);
    }

    #[test]
    fn test_multiply_underflow() {
        let color = Color::new(200, 200, 200);
        let result = color.multiply(-1.0);
        assert_eq!(result.red, 0);
        assert_eq!(result.green, 0);
        assert_eq!(result.blue, 0);
    }
}
