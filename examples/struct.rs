use bevy::prelude::*;
use bevy_contrib_inspector::{AsHtml, Inspectable, InspectorPlugin};

#[derive(Inspectable, Default, Debug)]
struct Data {
    noise_settings: NoiseSettings,
    tuple_struct: TupleStruct,
}

#[derive(AsHtml, Debug)]
pub struct NoiseSettings {
    octaves: usize,
    frequency: f64,
    lacunarity: f64,
    persistence: f64,
    attenuation: f64,
}

#[derive(AsHtml, Debug, Default)]
pub struct TupleStruct(String, String);

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            octaves: 0,
            frequency: 1.0,
            lacunarity: std::f64::consts::PI * 2.0 / 3.0,
            persistence: 1.0,
            attenuation: 2.0,
        }
    }
}

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_system(log.system())
        .run();
}

fn log(data: ChangedRes<Data>) {
    dbg!(&*data);
}
