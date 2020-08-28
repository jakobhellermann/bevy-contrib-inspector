use bevy::prelude::*;

mod inspector_server;

pub use inspector_server::InspectorServer;

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(server.system())
            .add_system(check.system());
    }
}

fn server(mut commands: Commands) {
    let server = InspectorServer::start_in_background("localhost:9121").unwrap();
    commands.insert_resource(server);
}

fn check(server: Res<InspectorServer>) {
    if let Ok(e) = server.rx.try_recv() {
        dbg!(e);
    }
}
