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

    let _ = framebuffer.render_buffer("out.bmp");

    println!("Framebuffer rendered to out.bmp");
}
