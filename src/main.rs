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

    // Poligono 1
    let poligon_1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), 
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    // Rellenar de amarillo
    fill_polygon(&poligon_1, &mut framebuffer, 0x00FFFF);
    
    // Dibujar el contorno blanco
    draw_polygon(&poligon_1, &mut framebuffer, 0xFFFFFF);
    
    // Poligono 2
    let poligon_2 = [
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    // Rellenar de azul
    fill_polygon(&poligon_2, &mut framebuffer, 0xFF0000);
    
    // Dibujar el contorno blanco
    draw_polygon(&poligon_2, &mut framebuffer, 0xFFFFFF);

    // Poligono 3
    let poligon_3 = [
        (377, 249), (411, 197), (436, 249)
    ];

    // Rellenar de rojo
    fill_polygon(&poligon_3, &mut framebuffer, 0x0000FF);
    
    // Dibujar el contorno blanco
    draw_polygon(&poligon_3, &mut framebuffer, 0xFFFFFF);
    
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
