use bevy::prelude::*;
use bevy_inspector::{Inspectable, InspectorPlugin};

#[derive(Debug)]
enum TextColor {
    White,
    Green,
    Blue,
}
impl bevy_inspector::as_html::AsHtml for TextColor {
    type Err = String;
    type Options = ();
    const DEFAULT_OPTIONS: Self::Options = ();

    fn as_html(
        shared: bevy_inspector::as_html::SharedOptions<Self>,
        (): Self::Options,
        submit_fn: &'static str,
    ) -> String {
        let mut html = String::new();
        html.push_str(&format!(
            r#"
            <div class="row">
                <label class="cell text-right">{}:</label>
                <div class="cell">"#,
            shared.label,
        ));

        for field in &["White", "Green", "Blue"] {
            html.push_str(&format!(
                r#"
                <label>
                    <input type="radio" value="{value}" name="{name}" {checked} oninput="{}(this.value)"/>
                    {value}
                </label>
            "#,
                submit_fn,
                value = field,
                name = shared.label,
                checked=if format!("{:?}", shared.default) == *field { "checked" } else {""}
            ));
        }

        html.push_str(r#"</div></div>"#);
        html
    }

    fn parse(value: &str) -> Result<Self, Self::Err> {
        match value {
            "White" => Ok(TextColor::White),
            "Green" => Ok(TextColor::Green),
            "Blue" => Ok(TextColor::Blue),
            _ => Err(value.to_string()),
        }
    }
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
        dbg!(&*data);
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
