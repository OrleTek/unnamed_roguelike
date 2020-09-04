use super::{
    Terminal,
    Window,
    Vec4,
    vec4
};
use std::cmp::max;

pub struct BasicTerminal
{
    pub width : u32,
    pub height : u32,
    pub font_width : u32,
    pub font_height : u32,
    pub glyphs : Vec<u16>,
    pub fg : Vec<Vec4>,
    pub bg : Vec<Vec4>,
}

impl BasicTerminal {
    pub fn new ( size : (u32,u32) ) -> BasicTerminal {
        BasicTerminal
        {
            width : size.0,
            height : size.1,
            font_width : 16,
            font_height : 16,
            glyphs : vec![0; (size.0*size.1) as usize],
            fg : vec![vec4(0.0,0.0,0.0,0.0); (size.0*size.1) as usize],
            bg : vec![vec4(0.0,0.0,0.0,1.0); (size.0*size.1) as usize],
        }
    }

    pub fn set( &mut self, x : u32, y : u32, glyph : u16, fg : Vec4, bg : Vec4)
    {
        let index = (y * self.width + x) as usize;

        self.glyphs[index] = glyph;
        self.fg[index] = fg;
        self.bg[index] = bg;
    }

    pub fn clear(&mut self)
    {
        self.glyphs = vec![0; (self.width * self.height) as usize];
        self.fg = vec![vec4(0.0,0.0,0.0,0.0); (self.width * self.height) as usize];
        self.bg = vec![vec4(0.0,0.0,0.0,1.0); (self.width * self.height) as usize];
    }
}

impl Terminal for BasicTerminal
{
    fn width(&self)-> u32{
        self.width
    }

    fn height(&self)-> u32{
        self.height
    }

    fn glyph_index( &self, x: u32, y: u32) -> u16
    {
        let index = (y * self.width + x) as usize;
        self.glyphs[index%self.glyphs.len()]
    }

    fn glyph_fg( &self, x: u32, y: u32) -> Vec4
    {
        let index = (y * self.width + x) as usize;
        self.fg[index%self.glyphs.len()]
    }

    fn glyph_bg( &self, x: u32, y: u32) -> Vec4
    {
        let index = (y * self.width + x) as usize;
        self.bg[index%self.glyphs.len()]
    }

    fn resize( &mut self, w: u32, h: u32)
    {
        self.width = max(4,w);
        self.height = max(4,h);
    }

    fn resize_to_fit( &mut self, window: &Window)
    {
        self.resize(window.width/self.font_width, window.height/self.font_height);
    }
}