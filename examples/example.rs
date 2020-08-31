use bevy::prelude::*;
use bevy_contrib_inspector::{AsHtml, Inspectable, InspectorPlugin};

#[derive(AsHtml, Debug)]
enum TextColor {
    White,
    Green,
    Blue,
}

#[derive(Inspectable, Debug)]
struct Data {
    #[inspectable(min = 10.0, max = 70.0)]
    font_size: f32,
    text: String,
    show_square: bool,
    text_color: TextColor,
    color: Color,
    position: Vec2,
}
impl Default for Data {
    fn default() -> Self {
        Data {
            font_size: 50.0,
            text: "Hello World!".to_string(),
            show_square: true,
            text_color: TextColor::White,
            color: Color::BLUE,
            position: Vec2::default(),
        }
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .add_system(shape_update_system.system())
        .run();
}

fn text_update_system(data: Res<Data>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("{}", data.text);
        text.style.font_size = data.font_size;
        text.style.color = match &data.text_color {
            TextColor::White => Color::WHITE,
            TextColor::Green => Color::GREEN,
            TextColor::Blue => Color::BLUE,
        };
    }
}

fn shape_update_system(
    data: Res<Data>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Handle<ColorMaterial>, &mut Translation)>,
) {
    for (color, mut pos) in &mut query.iter() {
        let material = materials.get_mut(&color).unwrap();
        material.color = data.color;

        if !data.show_square {
            pos.0.set_x(1000000.0)
        } else {
            pos.0.set_x(data.position.x() * 200.0);
            pos.0.set_y(data.position.y() * 200.0);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server
        .load("/usr/share/fonts/truetype/noto/NotoMono-Regular.ttf")
        .unwrap();

    let color = materials.add(Color::BLUE.into());

    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera2dComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(SpriteComponents {
            material: color,
            sprite: Sprite {
                size: Vec2::new(40.0, 40.0),
            },
            translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        });
}
