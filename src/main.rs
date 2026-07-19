mod framebuffer;
mod line;
mod bmp;
mod polygon_fill;

use crate::bmp::WriteBmp;
use crate::framebuffer::Framebuffer;
use crate::polygon_fill::{draw_polygon, fill_polygon, fill_polygon_with_hole};

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Poligono 4
    let poligon_4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37),
        (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214),
        (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    // Poligono 5 
    let poligon_5 = [
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    // Rellenar de verde
    fill_polygon_with_hole(&poligon_4, &poligon_5, &mut framebuffer, 0x00FF00);
    
    // Dibujar el contorno blanco de ambos polígonos
    draw_polygon(&poligon_4, &mut framebuffer, 0xFFFFFF);
    draw_polygon(&poligon_5, &mut framebuffer, 0xFFFFFF);

    let _ = framebuffer.render_buffer("out.bmp");

    println!("Framebuffer rendered to out.bmp");
}
