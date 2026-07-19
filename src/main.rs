mod framebuffer;
mod line;
mod bmp;
mod polygon_fill;

use crate::bmp::WriteBmp;
use crate::framebuffer::Framebuffer;
use crate::polygon_fill::{draw_polygon, fill_polygon};

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Poligono 3
    let poligon_3 = [
        (377, 249), (411, 197), (436, 249)
    ];

    // Rellenar de rojo
    fill_polygon(&poligon_3, &mut framebuffer, 0x0000FF);
    
    // Dibujar el contorno blanco
    draw_polygon(&poligon_3, &mut framebuffer, 0xFFFFFF);

    let _ = framebuffer.render_buffer("out.bmp");

    println!("Framebuffer rendered to out.bmp");
}
