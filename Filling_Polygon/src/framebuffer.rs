use crate::bmp::Bitmap;
use crate::color::Color;
use crate::vertex::Vertex;
use crate::line_impl::Line;  // Importamos el trait Line

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn draw_polygon(&mut self, vertices: &[Vertex]) {
        if vertices.len() < 2 {
            return;
        }

        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = if i == vertices.len() - 1 {
                vertices[0]
            } else {
                vertices[i + 1]
            };

            self.draw_line(start, end);  // Usamos el método draw_line del trait Line
        }
    }

    pub fn fill_polygon(&mut self, vertices: &[Vertex], fill_color: Color) {
        if vertices.len() < 3 {
            return;
        }

        let (min_y, max_y) = vertices.iter()
            .map(|v| v.y)
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_y, max_y), y| {
                (min_y.min(y), max_y.max(y))
            });

        for y in min_y.ceil() as usize..=max_y.floor() as usize {
            let mut intersections: Vec<f32> = Vec::new();

            for i in 0..vertices.len() {
                let j = if i == vertices.len() - 1 { 0 } else { i + 1 };

                let v1 = vertices[i];
                let v2 = vertices[j];

                if (v1.y <= y as f32 && v2.y > y as f32) || (v2.y <= y as f32 && v1.y > y as f32) {
                    let x_intersection = v1.x + (y as f32 - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                    intersections.push(x_intersection);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x_start = intersections[i].ceil() as isize;
                    let x_end = intersections[i + 1].floor() as isize;

                    for x in x_start..=x_end {
                        self.set_pixel(x, y as isize, fill_color);
                    }
                }
            }
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(255, 255, 255);  // Fondo blanco
        let buffer = vec![background_color; width * height];
        Self {
            width,
            height,
            buffer,
            background_color,
            current_color: Color::new(0, 0, 0),  // Color de dibujo negro
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = color;
        }
    }

    pub fn set_background_color(&mut self, hex_color: u32) {
        self.background_color = Color::from_hex(hex_color);
    }

    pub fn set_current_color(&mut self, hex_color: u32) {
        self.current_color = Color::from_hex(hex_color);
    }

    pub fn render_buffer(&self, filename: &str) -> std::io::Result<()> {
        let mut bitmap = Bitmap::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.buffer[y * self.width + x];
                bitmap.set_pixel(x as u32, (self.height - 1 - y) as u32, (color.red, color.green, color.blue));
            }
        }

        bitmap.save(filename)
    }

    pub fn get_buffer(&self) -> &Vec<Color> {
        &self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_framebuffer() {
        let mut framebuffer = Framebuffer::new(10, 10);
        let background_color = Color::from_hex(0xFFFFFF);  // Fondo blanco
        let foreground_color = Color::from_hex(0x000000);  // Color de dibujo negro

        framebuffer.set_background_color(0xFFFFFF);
        framebuffer.clear();

        for pixel in framebuffer.buffer.iter() {
            assert_eq!(*pixel, background_color);
        }

        framebuffer.set_current_color(0x000000);
        framebuffer.point(5, 5);

        assert_eq!(framebuffer.buffer[5 * 10 + 5], foreground_color);
    }
}
