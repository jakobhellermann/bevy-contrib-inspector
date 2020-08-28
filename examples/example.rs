use bevy::prelude::*;
use bevy_inspector::{Inspectable, InspectorPlugin};

#[derive(Default)]
struct Data {
    slider: u8,
}
impl Inspectable for Data {
    fn update(&mut self, field: &str, value: String) {
        match field {
            "slider" => match value.parse() {
                Ok(val) => self.slider = val,
                Err(e) => eprintln!("failed to parse 'slider' value '{}': {}", value, e),
            },
            _ => eprintln!("unexpected field '{}'", field),
        }
    }

    fn html() -> std::borrow::Cow<'static, str> {
        include_str!("../index.html").into()
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
        text.value = format!("time: {}", data.slider);
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
