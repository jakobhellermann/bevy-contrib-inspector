use bevy_inspector::InspectorServer;

fn main() -> std::io::Result<()> {
    let server = InspectorServer::start_in_background("localhost:1234")?;
    for event in server.rx {
        dbg!(event);
    }

    Ok(())
}
