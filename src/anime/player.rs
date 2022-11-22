extern crate rust_cast;
use rust_cast::{
    channels::{
        media::{Media, StreamType},
        receiver::CastDeviceApp,
    },
    CastDevice,
};
use std::str::FromStr;

pub fn open_video(link: (String, String)) {
    let title = link.1;
    let title = title.replace("-", " ");
    let arg: String = format!("--force-media-title={}", title);
    let _ = std::process::Command::new("mpv")
        .arg(link.0)
        .arg(arg)
        .output()
        .expect("failed to open mpv");

    // clear terminal
}

const DEFAULT_DESTINATION_ID: &str = "receiver-0";
fn play_media(
    device: &CastDevice,
    app_to_run: &CastDeviceApp,
    media: String,
    media_type: String,
    media_stream_type: StreamType,
) {
    let app = device.receiver.launch_app(app_to_run).unwrap();

    device
        .connection
        .connect(app.transport_id.as_str())
        .unwrap();

    let _status = device
        .media
        .load(
            app.transport_id.as_str(),
            app.session_id.as_str(),
            &Media {
                content_id: media,
                content_type: media_type,
                stream_type: media_stream_type,
                duration: None,
                metadata: None,
            },
        )
        .unwrap();
}

pub fn open_cast(link: (String, String), ip: &str) {
    let cast_device = match CastDevice::connect_without_host_verification(ip, 8009) {
        Ok(cast_device) => cast_device,
        Err(err) => panic!("Could not establish connection with Cast Device: {:?}", err),
    };

    cast_device
        .connection
        .connect(DEFAULT_DESTINATION_ID.to_string())
        .unwrap();
    cast_device.heartbeat.ping().unwrap();

    // Play media and keep connection.

    let media_stream_type = match "none" {
        value @ "buffered" | value @ "live" | value @ "none" => {
            StreamType::from_str(value).unwrap()
        }
        _ => panic!("Unsupported stream type!"),
    };
    play_media(
        &cast_device,
        &CastDeviceApp::from_str("default").unwrap(),
        link.0.to_string(),
        "".to_string(),
        media_stream_type,
    );
}
