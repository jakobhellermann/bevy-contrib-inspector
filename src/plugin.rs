use bevy::prelude::*;

use crate::inspector_server::{InspectorServer, ServerConfig};
use crate::Inspectable;

#[derive(Default, Clone)]
pub struct InspectorPlugin<T> {
    marker: std::marker::PhantomData<T>,
}
impl<T> InspectorPlugin<T> {
    pub fn new() -> InspectorPlugin<T> {
        InspectorPlugin {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Inspectable> InspectorPlugin<T> {
    fn check(server: Res<InspectorServer>, mut inspectable_data: ResMut<T>) {
        if let Ok((field, data)) = server.rx.try_recv() {
            inspectable_data.update(&field, &data);
        }
    }

    fn start_server(commands: &mut Commands) {
        let config = ServerConfig::new(T::html());

        let options = T::options();

        let addr = format!("localhost:{}", options.port);
        let server = InspectorServer::start_in_background(&addr, config).unwrap();

        if should_open_browser() {
            if let Err(e) = open_inspector_window(format!("http://{}", addr)) {
                eprintln!("failed to open {}: {}", addr, e);
            }
        }

        commands.insert_resource(server);
    }
}

#[cfg(not(feature = "native"))]
fn open_inspector_window(addr: String) -> Result<(), std::io::Error> {
    webbrowser::open(&addr)?;
    Ok(())
}
#[cfg(feature = "native")]
fn open_inspector_window(addr: String) -> Result<(), std::convert::Infallible> {
    std::thread::spawn(move || {
        webview_official::WebviewBuilder::new()
            .width(800)
            .height(600)
            .url(&addr)
            .build()
            .run();
        /*web_view::builder()
        .title("Bevy Inspector")
        .content(web_view::Content::Url(addr))
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();*/
    });

    Ok(())
}

#[cfg(feature = "native")]
fn should_open_browser() -> bool {
    true
}

#[cfg(not(feature = "native"))]
fn should_open_browser() -> bool {
    let is_confirmation = |var: String| match var.as_str() {
        "1" | "true" | "yes" | "y" => true,
        "0" | "false" | "no" | "n" => false,
        other => {
            eprintln!("unexpected value for BEVY_INSPECTOR_OPEN: {}", other);
            false
        }
    };

    let variable =
        std::env::var("BEVY_INSPECTOR_OPEN").or_else(|_| std::env::var("BEVY_OPEN_INSPECTOR"));
    variable.map_or(false, is_confirmation)
}

impl<T: Inspectable + Default> Plugin for InspectorPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(T::default())
            .add_startup_system(Self::start_server.system())
            .add_system(Self::check.system());
    }
}
