mod framebuffer;
mod line;
mod bmp;

use crate::bmp::WriteBmp;
use crate::framebuffer::Framebuffer;
use crate::line::draw_line;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);

    let _ = framebuffer.render_buffer("out.bmp");

    println!("Framebuffer rendered to out.bmp");
}
