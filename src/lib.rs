mod html_impls;
mod inspector_server;
mod plugin;

pub use bevy_inspector_derive::Inspectable;
pub use plugin::InspectorPlugin;

pub trait Inspectable: Send + Sync + 'static {
    fn html() -> String;
    fn update(&mut self, field: &str, value: String);
    fn options() -> InspectableOptions {
        InspectableOptions::default()
    }
}

pub struct InspectableOptions {
    pub port: u16,
}
impl Default for InspectableOptions {
    fn default() -> Self {
        InspectableOptions { port: 8668 }
    }
}

pub trait AsHtml: Sized {
    type Options;
    const DEFAULT_OPTIONS: Self::Options;

    fn as_html(options: Self::Options, submit_fn: &'static str) -> String;
    fn parse(value: &str) -> Result<Self, ()>;
}
