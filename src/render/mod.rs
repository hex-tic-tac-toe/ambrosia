use crate::game::{board::Board, hex::Hex, player::Player};

use std::f32::consts::PI;
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

/// Flat-top hex layout. `size` is the circumradius (center to corner) in pixels.
fn hex_corners_flat(cx: f32, cy: f32, size: f32) -> [(f32, f32); 6] {
    std::array::from_fn(|i| {
        let angle = PI / 180.0 * (60.0 * i as f32); // flat-top: 0°, 60°, 120°...
        (cx + size * angle.cos(), cy + size * angle.sin())
    })
}

/// Axial (q, r) → pixel center, flat-top layout.
fn hex_to_pixel(q: i32, r: i32, size: f32, origin: (f32, f32)) -> (f32, f32) {
    let x = origin.0 + size * (3.0 / 2.0 * q as f32);
    let y = origin.1 + size * (3_f32.sqrt() / 2.0 * q as f32 + 3_f32.sqrt() * r as f32);
    (x, y)
}

pub fn render_board(
    c: impl Iterator<Item = (Hex, Player)>,
    output_path: &str,
    winning_line: Vec<Hex>,
) {
    let cells: Vec<(Hex, Player)> = c.collect();
    let hex_size = 5.0;
    let padding = 5.0;
    let bg = Color::from_rgba8(255, 255, 255, 255);
    let border = Color::from_rgba8(0, 0, 0, 255);
    let border_winner = Color::from_rgba8(127, 127, 127, 255);
    let border_width = 1.0;
    // Compute bounding box over all cell centers.
    let centers: Vec<(f32, f32)> = cells
        .iter()
        .map(|c| hex_to_pixel(c.0.0, c.0.1, hex_size, (0.0, 0.0)))
        .collect();

    let min_x = centers.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
    let min_y = centers.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
    let max_x = centers
        .iter()
        .map(|p| p.0)
        .fold(f32::NEG_INFINITY, f32::max);
    let max_y = centers
        .iter()
        .map(|p| p.1)
        .fold(f32::NEG_INFINITY, f32::max);

    // Shift origin so all cells sit within positive coordinates + padding.
    let origin = (-min_x + padding + hex_size, -min_y + padding + hex_size);

    let width = (max_x - min_x + 2.0 * (padding + hex_size)) as u32;
    let height = (max_y - min_y + 2.0 * (padding + hex_size)) as u32;

    let mut pixmap = Pixmap::new(width, height).expect("image too large");
    pixmap.fill(bg);

    for (cell, &(raw_cx, raw_cy)) in cells.iter().zip(centers.iter()) {
        let cx = raw_cx + origin.0;
        let cy = raw_cy + origin.1;
        let corners = hex_corners_flat(cx, cy, hex_size);

        // Build filled hex path.
        let mut pb = PathBuilder::new();
        pb.move_to(corners[0].0, corners[0].1);
        for &(x, y) in &corners[1..] {
            pb.line_to(x, y);
        }
        pb.close();
        let path = pb.finish().unwrap();

        // Fill.
        let mut paint = Paint::default();
        if winning_line.contains(&cell.0) {
            paint.set_color(match cell.1 {
                Player::X => Color::from_rgba8(255, 127, 127, 255),
                Player::O => Color::from_rgba8(127, 127, 255, 255),
            });
        } else {
            paint.set_color(match cell.1 {
                Player::X => Color::from_rgba8(255, 0, 0, 255),
                Player::O => Color::from_rgba8(0, 0, 255, 255),
            });
        }
        paint.anti_alias = true;
        pixmap.fill_path(
            &path,
            &paint,
            tiny_skia::FillRule::Winding,
            Transform::identity(),
            None,
        );

        // Border.
        if border_width > 0.0 {
            let mut stroke_paint = Paint::default();
            stroke_paint.set_color(border);
            stroke_paint.anti_alias = true;
            let stroke = Stroke {
                width: border_width,
                ..Default::default()
            };
            pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
        }
    }

    println!("{}x{}", pixmap.width(), pixmap.height());
    pixmap.save_png(output_path).unwrap();
}
