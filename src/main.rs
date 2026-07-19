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

    // Poligono 2
    let poligon_2 = [
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    // Rellenar de azul
    fill_polygon(&poligon_2, &mut framebuffer, 0xFF0000);
    
    // Dibujar el contorno blanco
    draw_polygon(&poligon_2, &mut framebuffer, 0xFFFFFF);

    let _ = framebuffer.render_buffer("out.bmp");

    println!("Framebuffer rendered to out.bmp");
}
