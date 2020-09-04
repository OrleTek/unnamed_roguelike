

use super::{
    Terminal,
    VertexAttribute,

};

pub fn terminal_mesh( terminal : &dyn Terminal  ) -> (Vec<VertexAttribute>, Option<Vec<u32>>) {
    let width: u32 = terminal.width();
    let height: u32 = terminal.height();

    let mut indices = Vec::with_capacity((width * height * 6) as usize);

    let mut positions = Vec::with_capacity((width * height * 4) as usize);
    let mut normals = Vec::with_capacity((width * height * 4) as usize);
    let mut uvs = Vec::with_capacity((width * height * 4) as usize);

    let ox = width as f32/-2.0;
    let oy = height as f32/-2.0;

    let mut v = 0;
    for x in 0..width {
        for y in 0..height {
            positions.push([ox + x as f32, oy + y as f32, 0.0]); //sw
            positions.push([ox + x as f32, oy + 1.0 + y as f32, 0.0]); //nw
            positions.push([ox + 1.0 + x as f32, oy + y as f32, 0.0]); //se
            positions.push([ox + 1.0 + x as f32, oy + 1.0 + y as f32, 0.0]); //ne

            let glyph = terminal.glyph_index(x,y) as f32;
            let fg = (x + width * y) as f32 *2.0;
            let bg = fg+1.0;

            normals.push([glyph, fg, bg]);
            normals.push([glyph, fg, bg]);
            normals.push([glyph, fg, bg]);
            normals.push([glyph, fg, bg]);

            uvs.push([0.0, 1.0]);
            uvs.push([0.0, 0.0]);
            uvs.push([1.0, 1.0]);
            uvs.push([1.0, 0.0]);

            indices.push(v); //sw
            indices.push(v + 3); //ne
            indices.push(v + 1); //nw
            indices.push(v); //sw
            indices.push(v + 2); //se
            indices.push(v + 3); //ne
            v += 4;
        }
    }

    (vec![
        VertexAttribute::position(positions),
        VertexAttribute::normal(normals),
        VertexAttribute::uv(uvs),
    ],
    Some(indices))
}