use bevy::prelude::*;
use bevy_inspector::InspectorPlugin;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(InspectorPlugin)
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .run();
}

fn text_update_system(time: Res<Time>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("time: {:.2}", time.seconds_since_startup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server
        .load("/usr/share/fonts/truetype/noto/NotoMono-Regular.ttf")
        .unwrap();

    let text = "Test";

    commands
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: text.to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                ..Default::default()
            },
            ..Default::default()
        });
}
