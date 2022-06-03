use std::time::Duration;
use tokio::time::{sleep, timeout};
use rdr_core::message::detected_armor::CarInfo;
use rdr_core::prelude::*;
use rdr_zeromq::prelude::*;

use rdr_zeromq::client::*;
use rdr_zeromq::server::*;

use std::{env, fs};
use std::io;
use std::io::Read;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "server") { server().await.unwrap(); }
    else if args.iter().any(|arg| arg == "client") { client().await; }
    else { panic!("not server or client"); }
}

async fn server() -> io::Result<()> {
    let mut server = EncodedImgServer::new("tcp://127.0.0.1:5555").await;

    println!("Wait key to send image...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();

    let entries = fs::read_dir("/home/shiroki/Desktop/batch1_part5/")?
        .filter_map(|res| res.ok())
        .map(|res| res.path())
        .filter(|path| path.is_file() && path.extension() == Some(std::ffi::OsStr::new("jpg")))
        .collect::<Vec<_>>();

    for path in entries.iter() {
        let mut file = fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        server.send_img(Bytes::from(buf)).await.unwrap();

        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

async fn client() {
    let future_client = DetectedArmorClient::new("tcp://127.0.0.1:5556");
    let mut client = timeout(Duration::from_secs(10), future_client).await.unwrap();
    println!("Connected. Waiting for server to start...");
    client.socket_subscribe("").await.unwrap();
    while let Ok(data) = client.recv().await {
        if let Some(time) = data.timestamp.0 {
            print!("On {}: ", time);
        } else { panic!("A message without Timestamp!") }
        if data.armors.is_empty() {
            println!("No armors found");
        } else {
            println!();

            use detected_armor::car::Type::*;

            for car_type in [CAR, BASE, WATCHER].iter() {
                let by_car_type = |info: &&CarInfo| info.car.type_.unwrap() == *car_type;

                let count = data.armors.iter().filter(by_car_type).count();
                print!("{} {:?} found: ", count, car_type);

                let contact = data.armors.iter().filter(by_car_type)
                    .fold(String::new(), |acc, info|
                        if acc.is_empty() {
                            format!("{:?} {:?}", info.car.color.unwrap(), info.armor.type_.unwrap())
                        } else {
                            format!("{}, {:?} {:?}", acc, info.car.color.unwrap(), info.armor.type_.unwrap())
                        });
                println!("{}", contact);
            }
        }
    }
}