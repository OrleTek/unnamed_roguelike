
use bevy::prelude::*;

use super::{
    Terminal,
    ColorAtlas,
    vec2,
    vec4,
    TERMINAL_PIPELINE_HANDLE,
    PipelineSpecialization,
    RenderPipeline,
    BasicTerminal,
    DynamicBinding,
    terminal_mesh
};

pub fn setup_terminal_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut color_atlases: ResMut<Assets<ColorAtlas>>,
    
)
{
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/font_16.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 32, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let color_atlas = ColorAtlas { 
        size : vec2(1.0,1.0),
        colors : vec![vec4(0.0,1.0,0.0,1.0)]
    };
    let color_atlas_handle = color_atlases.add(color_atlas);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(
            SpriteSheetComponents {
                texture_atlas: texture_atlas_handle,
                mesh : meshes.add( Mesh::from(shape::Quad::new(vec2(32.0,32.0)))),
                render_pipelines: RenderPipelines::from_pipelines(
                    vec![RenderPipeline::specialized(
                        TERMINAL_PIPELINE_HANDLE,
                        PipelineSpecialization {
                            dynamic_bindings: vec![
                                // Transform
                                DynamicBinding {
                                    bind_group: 2,
                                    binding: 0,
                                },
                                // TextureAtlasSprite
                                DynamicBinding {
                                    bind_group: 2,
                                    binding: 1,
                                },
                            ],
                            ..Default::default()
                        },
                    )]
                ),
                ..Default::default()
            }
        ).with(
            color_atlas_handle
        )
        .with(BasicTerminal::new((32,32)) );
}

pub fn update_terminal_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut atlases: ResMut<Assets<ColorAtlas>>,
    windows : Res<Windows>,
    mut query: Query<(&mut BasicTerminal, &Handle<ColorAtlas>, &Handle<Mesh>)>,
    
) {
    for (mut term, atlas_handle, mesh_handle) in &mut query.iter() {
        let window = windows.get_primary().unwrap();

        term.resize_to_fit(window);

        let mesh = meshes.get_mut(mesh_handle).unwrap();
        let (attributes,indices) = terminal_mesh(&*term);
        mesh.attributes = attributes;
        mesh.indices = indices;

        let atlas = atlases.get_mut(atlas_handle).unwrap();
        atlas.size = vec2( term.width as f32, term.height as f32);
        atlas.colors.clear();
        
        for y in 0..term.height{
            for x in 0..term.width {
                atlas.colors.push(
                    term.glyph_fg(x,y)
                );
                atlas.colors.push(
                    term.glyph_bg(x,y)
                );
            }
        }
    }
}
