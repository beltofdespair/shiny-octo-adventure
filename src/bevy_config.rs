use crate::util::error;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
// use bevy::window::WindowMode;
// use bevy::window::WindowResolution;
use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use std::io::Cursor;
use winit::window::Icon;

/// Stores the various window-resolutions we can select between.
// #[derive(Resource)]
// struct ResolutionSettings {
//     large: Vec2,
//     medium: Vec2,
//     small: Vec2,
// }

/// Overrides the default Bevy plugins and configures things like the screen
/// settings.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // resizable: false,
                    // resolution: WindowResolution::new(1920.0, 1080.0)
                    //     .with_scale_factor_override(2.0),
                    // mode: WindowMode::BorderlessFullscreen,
                    title: "Shiny Octo Adventure".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: create_wgpu_settings().into(),
                synchronous_pipeline_compilation: false,
            }),
    )
    // .insert_resource(Msaa::Sample4)
    // .insert_resource(ResolutionSettings {
    //     large: Vec2::new(1920.0, 1080.0),
    //     medium: Vec2::new(1280.0, 720.0),
    //     small: Vec2::new(640.0, 360.0),
    // })
    .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    // .add_systems(Update, toggle_resolution)
    .add_systems(Startup, set_window_icon.pipe(error));
}

fn create_wgpu_settings() -> WgpuSettings {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    wgpu_settings
}

// Sets the icon on Windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
) -> anyhow::Result<()> {
    let primary_entity = primary_windows.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return Ok(());
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height)?;
        primary.set_window_icon(Some(icon));
    };
    Ok(())
}

// / This system shows how to request the window to a new resolution
// fn toggle_resolution(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut windows: Query<&mut Window>,
//     resolution: Res<ResolutionSettings>,
// ) {
//     let mut window = windows.single_mut();

//     if keys.just_pressed(KeyCode::Digit1) {
//         let res = resolution.small;
//         window.resolution.set(res.x, res.y);
//     }
//     if keys.just_pressed(KeyCode::Digit2) {
//         let res = resolution.medium;
//         window.resolution.set(res.x, res.y);
//     }
//     if keys.just_pressed(KeyCode::Digit3) {
//         let res = resolution.large;
//         window.resolution.set(res.x, res.y);
//     }
// }
