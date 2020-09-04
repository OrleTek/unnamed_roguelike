use bevy::prelude::*;

//use super::Vertex;
use bevy_math::*;



use bevy::render::{
    pipeline::{
        DynamicBinding, PipelineSpecialization, RenderPipeline
    },
    render_graph::RenderGraph,
    
};

pub mod renderer;
use renderer::TerminalRenderGraphBuilder;
use renderer::TERMINAL_PIPELINE_HANDLE;
use renderer::ColorAtlas;

pub mod terminal;
use terminal::BasicTerminal;
use terminal::TerminalPlugin;
