use bevy::prelude::*;

use bevy::diagnostic::{ Diagnostics, FrameTimeDiagnosticsPlugin};

use display::terminal::TerminalPlugin;
use display::terminal::BasicTerminal;

use bevy::math::vec4;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(TerminalPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(test_display.system())
        .run();
}

fn test_display(diagnostics: Res<Diagnostics>,
    mut query: Query<&mut BasicTerminal>,
    
) {
    for mut term in &mut query.iter() {
        term.clear();
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                let y = 6;
                let mut x = 6;
                for c in average.to_string().chars() {
                    term.set(x,y,match c {
                        '0' => 48,
                        '1' => 49,
                        '2' => 50,
                        '3' => 51,
                        '4' => 52,
                        '5' => 53,
                        '6' => 54,
                        '7' => 55,
                        '8' => 56,
                        '9' => 57,
                        _ => 63,
                    }, vec4(1.0,0.0,1.0,1.0),
                    vec4(0.0,0.0,0.0,1.0));
                    x+=1;
                }
            }
        }
    }
}