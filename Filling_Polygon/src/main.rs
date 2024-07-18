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

    // Define the vertices of the polygon 4
    let vertices_4 = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0),
    ];

    // Define the vertices of the polygon 5 (hole)
    let vertices_5 = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0),
    ];

    // Fill the polygon 4 with green color
    framebuffer.fill_polygon(&vertices_4, Color::new(0, 255, 0));
    // Draw the polygon 4's edges with white color
    framebuffer.draw_polygon(&vertices_4);

    // Fill the polygon 5 (hole) with the background color
    framebuffer.fill_polygon(&vertices_5, Color::new(0, 0, 0));
    // Draw the polygon 5's edges with white color
    framebuffer.draw_polygon(&vertices_5);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("out.bmp").expect("Failed to render buffer");
}
