use crate::framebuffer::Framebuffer;
use crate::line::draw_line;

pub fn draw_polygon(points: &[(i32, i32)], framebuffer: &mut Framebuffer, color: u32) {
    if points.is_empty() {
        return;
    }

    framebuffer.set_current_color(color);

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        draw_line(p1.0, p1.1, p2.0, p2.1, framebuffer);
    }
}

pub fn fill_polygon(points: &[(i32, i32)], framebuffer: &mut Framebuffer, fill_color: u32) {
    if points.is_empty() {
        return;
    }

    // Encontrar el Y mínimo y el Y máximo del polígono
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    for point in points {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    // Iterar sobre cada línea horizontal (Scanline)
    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        // Encontrar las intersecciones de esta línea y con las aristas del polígono
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            // Comprobar si la línea horizontal 'y' cruza la arista
            let y_min = p1.1.min(p2.1);
            let y_max = p1.1.max(p2.1);

            if y >= y_min && y < y_max {
                // Calcular la coordenada X de la intersección
                let dx = p2.0 as f32 - p1.0 as f32;
                let dy = p2.1 as f32 - p1.1 as f32;
                
                if dy != 0.0 {
                    let x_int = p1.0 as f32 + (y as f32 - p1.1 as f32) * dx / dy;
                    intersections.push(x_int.round() as i32);
                }
            }
        }

        // Ordenar las intersecciones de izquierda a derecha
        intersections.sort();

        // Rellenar los píxeles entre los pares de intersecciones
        framebuffer.set_current_color(fill_color);
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];

                for x in x_start..=x_end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}

pub fn fill_polygon_with_hole(exterior: &[(i32, i32)], hole: &[(i32, i32)], framebuffer: &mut Framebuffer, fill_color: u32) {
    if exterior.is_empty() {
        return;
    }
    let mut min_y = exterior[0].1;
    let mut max_y = exterior[0].1;
    for point in exterior.iter().chain(hole.iter()) {
        if point.1 < min_y { min_y = point.1; }
        if point.1 > max_y { max_y = point.1; }
    }
    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        // Intersecciones del polígono exterior
        for i in 0..exterior.len() {
            let p1 = exterior[i];
            let p2 = exterior[(i + 1) % exterior.len()];
            let y_min = p1.1.min(p2.1);
            let y_max = p1.1.max(p2.1);
            if y >= y_min && y < y_max {
                let dx = p2.0 as f32 - p1.0 as f32;
                let dy = p2.1 as f32 - p1.1 as f32;
                if dy != 0.0 {
                    let x_int = p1.0 as f32 + (y as f32 - p1.1 as f32) * dx / dy;
                    intersections.push(x_int.round() as i32);
                }
            }
        }
        // Intersecciones del agujero
        for i in 0..hole.len() {
            let p1 = hole[i];
            let p2 = hole[(i + 1) % hole.len()];
            let y_min = p1.1.min(p2.1);
            let y_max = p1.1.max(p2.1);
            if y >= y_min && y < y_max {
                let dx = p2.0 as f32 - p1.0 as f32;
                let dy = p2.1 as f32 - p1.1 as f32;
                if dy != 0.0 {
                    let x_int = p1.0 as f32 + (y as f32 - p1.1 as f32) * dx / dy;
                    intersections.push(x_int.round() as i32);
                }
            }
        }
        intersections.sort();
        framebuffer.set_current_color(fill_color);
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
