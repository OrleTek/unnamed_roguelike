use bevy::{
    asset::{Assets, Handle},
    ecs::Resources,
    render::{
        pipeline::{
            BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite,
            CompareFunction, CullMode, DepthStencilStateDescriptor, FrontFace, PipelineDescriptor,
            RasterizationStateDescriptor, StencilStateDescriptor, StencilStateFaceDescriptor,
        },
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
        renderer::{RenderResources},
    },
    sprite::{ColorMaterial, TextureAtlas},
    math::{Vec2,Vec4}
};

pub const TERMINAL_PIPELINE_HANDLE: Handle<PipelineDescriptor> =
    Handle::from_u128(234908902690993092322946390028479586852);

pub fn build_terminal_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        color_states: vec![ColorStateDescriptor {
            format: TextureFormat::Bgra8UnormSrgb,
            color_blend: BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("terminal.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("terminal.frag"),
            ))),
        })
    }
}

pub mod node {
    pub const COLOR_MATERIAL: &str = "color_material";
    pub const TERMINAL: &str = "terminal";
    pub const TERMINAL_COLORS: &str = "terminal_colors";
    pub const TERMINAL_SPRITE: &str = "terminal_sprite";
}
pub trait TerminalRenderGraphBuilder {
    fn add_terminal_graph(&mut self, resources: &Resources) -> &mut Self;
}

#[derive(RenderResources)]
pub struct ColorAtlas {
    pub size : Vec2,
    #[render_resources(buffer)]
    pub colors: Vec<Vec4>,
}

impl TerminalRenderGraphBuilder for RenderGraph {
    fn add_terminal_graph(&mut self, resources: &Resources) -> &mut Self {
        self.add_system_node(
            node::COLOR_MATERIAL,
            AssetRenderResourcesNode::<ColorMaterial>::new(false),
        );
        self.add_node_edge(node::COLOR_MATERIAL, base::node::MAIN_PASS)
            .unwrap();

        self.add_system_node(
            node::TERMINAL,
            AssetRenderResourcesNode::<TextureAtlas>::new(false),
        );

        self.add_system_node(
            node::TERMINAL_COLORS,
            AssetRenderResourcesNode::<ColorAtlas>::new(false),
        );

        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
        pipelines.set(
            TERMINAL_PIPELINE_HANDLE,
            build_terminal_pipeline(&mut shaders),
        );
        self
    }
}
