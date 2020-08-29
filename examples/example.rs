use bevy::prelude::*;
use bevy_inspector::{AsHtml, Inspectable, InspectorPlugin};

#[derive(AsHtml, Debug)]
enum TextColor {
    White,
    Green,
    Blue,
}

#[derive(Inspectable, Debug)]
#[inspectable(port = 8668)]
struct Data {
    #[inspectable(min = 10.0, max = 70.0)]
    font_size: f32,
    text: String,
    black: bool,
    text_color: TextColor,
}
impl Default for Data {
    fn default() -> Self {
        Data {
            font_size: 50.0,
            text: "Hello World!".to_string(),
            black: false,
            text_color: TextColor::White,
        }
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .run();
}

fn text_update_system(data: Res<Data>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Text: {}", data.text);
        text.style.color = match (data.black, &data.text_color) {
            (true, _) => Color::BLACK,
            (false, TextColor::White) => Color::WHITE,
            (false, TextColor::Green) => Color::GREEN,
            (false, TextColor::Blue) => Color::BLUE,
        };
        text.style.font_size = data.font_size;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server
        .load("/usr/share/fonts/truetype/noto/NotoMono-Regular.ttf")
        .unwrap();

    let text = "Text:";

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
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                ..Default::default()
            },
            ..Default::default()
        });
}
