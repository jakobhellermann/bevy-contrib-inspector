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
            inspectable_data.update(&field, data);
        }
    }

    fn start_server(mut commands: Commands) {
        let config = ServerConfig::new(T::html());

        let options = T::options();
        let server = InspectorServer::start_in_background(options.port, config).unwrap();
        commands.insert_resource(server);
    }
}

impl<T: Inspectable + Default> Plugin for InspectorPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(T::default())
            .add_startup_system(Self::start_server.system())
            .add_system(Self::check.system());
    }
}
