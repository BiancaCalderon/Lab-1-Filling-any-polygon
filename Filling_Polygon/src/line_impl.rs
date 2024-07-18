use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::vertex::VertexExt;
use nalgebra_glm::Vec3;

pub trait Line {
    fn draw_line(&mut self, start: Vertex, end: Vertex);
}

impl Line for Framebuffer {
    fn draw_line(&mut self, start: Vertex, end: Vertex) {
        let mut x = start.x;
        let mut y = start.y;
        let x1 = end.x;
        let y1 = end.y;

        let dx = (x1 - x).abs();
        let dy = (y1 - y).abs();
        let sx = if x < x1 { 1.0 } else { -1.0 };
        let sy = if y < y1 { 1.0 } else { -1.0 };
        let mut err = dx - dy;

        loop {
            self.point(x as isize, y as isize);
            if x == x1 && y == y1 { break; }
            let e2 = 2.0 * err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 < dx { err += dx; y += sy; }
        }
    }
}
