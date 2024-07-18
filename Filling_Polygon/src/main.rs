mod framebuffer;
mod line_impl;
mod bmp;
mod vertex;
mod color;

use framebuffer::Framebuffer;
use nalgebra_glm::Vec3;
use color::Color;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0x0000000);
    framebuffer.clear();

    // Set the current drawing color to white for the borders
    framebuffer.set_current_color(0xFFFFFF);

    // Define the vertices of the polygon 1
    let vertices_1 = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    // Fill the polygon 1 with yellow color
    framebuffer.fill_polygon(&vertices_1, Color::new(255, 255, 0));
    // Draw the polygon 1's edges with white color
    framebuffer.draw_polygon(&vertices_1);

    // Define the vertices of the polygon 2
    let vertices_2 = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];

    // Fill the polygon 2 with blue color
    framebuffer.fill_polygon(&vertices_2, Color::new(0, 0, 255));
    // Draw the polygon 2's edges with white color
    framebuffer.draw_polygon(&vertices_2);

    // Define the vertices of the polygon 3
    let vertices_3 = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0),
    ];

    // Fill the polygon 3 with red color
    framebuffer.fill_polygon(&vertices_3, Color::new(255, 0, 0));
    // Draw the polygon 3's edges with white color
    framebuffer.draw_polygon(&vertices_3);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("out.bmp").expect("Failed to render buffer");
}
