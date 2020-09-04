pub mod mesh;

use super::{
    Plugin,
    AppBuilder,
    ColorAtlas,
    vec2,
    vec4,
    Vec4,
    TERMINAL_PIPELINE_HANDLE,
    PipelineSpecialization,
    RenderPipeline,
    DynamicBinding,
    RenderGraph,
    TerminalRenderGraphBuilder
};

use bevy::{
    render::mesh::VertexAttribute,
    window::Window,
    prelude::*
};

use mesh::terminal_mesh;

mod basic_terminal;
pub use basic_terminal::BasicTerminal;

mod system;


pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ColorAtlas>()
            .add_startup_system( system::setup_terminal_system.system() )
            .add_system(system::update_terminal_system.system());

        {
            let resources = app.resources();
            let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
            render_graph.add_terminal_graph(resources);
        }
    }
}

pub trait Terminal {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn glyph_index( &self, x: u32, y: u32) -> u16;

    fn glyph_fg( &self, x: u32, y: u32) -> Vec4;
    fn glyph_bg( &self, x: u32, y: u32) -> Vec4;

    fn resize( &mut self, w: u32, h: u32);

    fn resize_to_fit( &mut self, window: &Window);
}