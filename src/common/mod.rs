use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use enigo::{Enigo, MouseControllable, MouseButton};

use tungstenite::{accept, Message};

pub mod key;
pub mod mouse;

#[derive(Debug, PartialEq)]
pub enum BrowserEvent {
    KeyDown(String),
    KeyUp(String),
    MouseDown(String),
    MouseUp(String),
    MouseMove(((i32, i32), (i32, i32))),
    MouseWheel((i32, i32)),
    Open,
    Close,
}


#[allow(clippy::similar_names)]
fn handle_connection(stream: TcpStream, tx: &Sender<BrowserEvent>) {
    let mut websocket = accept(stream).unwrap();

    println!("Start waiting for messages");
    loop {
        let message = websocket.read_message().unwrap();
        // println!("Start processing message");

        match message {
            Message::Close(_) => {
                println!("Message::Close received");
                tx.send(BrowserEvent::Close).unwrap();
                println!("Client disconnected");
                return;
            }
            Message::Text(msg) => {
                // println!("Message::Text received");
                println!("msg: {msg:?}");
                let (key, data) = msg.split_once(':').unwrap();
                let be = match key {
                    "open" => BrowserEvent::Open,
                    "close" => BrowserEvent::Close, // Is this needed?
                    "keydown" => BrowserEvent::KeyDown(data.to_string()),
                    "keyup" => BrowserEvent::KeyUp(data.to_string()),
                    "mousedown" => BrowserEvent::MouseDown(data.to_string()),
                    "mouseup" => BrowserEvent::MouseUp(data.to_string()),
                    "mousemove" => {
                        let (x, y) = data.split_once(',').unwrap();
                        Enigo::new().mouse_move_to(x.parse().unwrap(), y.parse().unwrap());
                        continue;
                    },
                    "onlefttap"=>{
                        Enigo::new().mouse_click(MouseButton::Left);
                        continue;
                    }
                    "onrighttap"=>{
                        Enigo::new().mouse_click(MouseButton::Right);
                        continue;
                    }
                     "mouse_move_relative" => {
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        let (x, y) = data.split_once(',').unwrap();
                        Enigo::new().mouse_move_relative(x.parse().unwrap(), y.parse().unwrap());
                        continue;
                    },
                    "mousewheel" => {
                        // format is x,y
                        let (x, y) = data.split_once(',').unwrap();
                        BrowserEvent::MouseWheel((x.parse().unwrap(), y.parse().unwrap()))
                    }
                    _ => {
                        println!("Other text received");
                        continue;
                    }
                };
                // tx.send(be).unwrap();
            }
            _ => {
                println!("Other Message received");
            }
        }
    }
}

#[allow(clippy::similar_names)]
fn handle_raw_connection(stream: TcpStream, tx: &Sender<BrowserEvent>) {
    let mut buf = [0; 128];
    loop {
        // 读取内容
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            println!("ok");
            break;
        }
        // 输出读取到的内容
        println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
    }
    let mut websocket = accept(stream).unwrap();

    println!("Start waiting for messages");
    loop {
        let message = websocket.read_message().unwrap();
        // println!("Start processing message");

        match message {
            Message::Close(_) => {
                println!("Message::Close received");
                tx.send(BrowserEvent::Close).unwrap();
                println!("Client disconnected");
                return;
            }
            Message::Text(msg) => {
                // println!("Message::Text received");
                println!("msg: {msg:?}");
                let (key, data) = msg.split_once(':').unwrap();
                let be = match key {
                    "open" => BrowserEvent::Open,
                    "close" => BrowserEvent::Close, // Is this needed?
                    "keydown" => BrowserEvent::KeyDown(data.to_string()),
                    "keyup" => BrowserEvent::KeyUp(data.to_string()),
                    "mousedown" => BrowserEvent::MouseDown(data.to_string()),
                    "mouseup" => BrowserEvent::MouseUp(data.to_string()),
                    "mousemove" => {
                        let (x, y) = data.split_once(',').unwrap();
                        Enigo::new().mouse_move_to(x.parse().unwrap(), y.parse().unwrap());
                        continue;
                    },
                    "onlefttap"=>{
                        Enigo::new().mouse_click(MouseButton::Left);
                        continue;
                    }
                    "onrighttap"=>{
                        Enigo::new().mouse_click(MouseButton::Right);
                        continue;
                    }
                     "mouse_move_relative" => {
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        // Enigo::new().mouse_move_relative(1, 1);
                        let (x, y) = data.split_once(',').unwrap();
                        Enigo::new().mouse_move_relative(x.parse().unwrap(), y.parse().unwrap());
                        continue;
                    },
                    "mousewheel" => {
                        // format is x,y
                        let (x, y) = data.split_once(',').unwrap();
                        BrowserEvent::MouseWheel((x.parse().unwrap(), y.parse().unwrap()))
                    }
                    _ => {
                        println!("Other text received");
                        continue;
                    }
                };
                // tx.send(be).unwrap();
            }
            _ => {
                println!("Other Message received");
            }
        }
    }
}
pub fn launch_ws_server(tx: Sender<BrowserEvent>) {
    let listener = TcpListener::bind("0.0.0.0:26541").unwrap();
    println!("TcpListener was created");

    match listener.accept() {
        Ok((stream, addr)) => {
            println!("New connection was made from {addr:?}");
            std::thread::spawn(move || handle_raw_connection(stream, &tx));
        }
        Err(e) => {
            println!("Connection failed: {e:?}");
        }
    }
}
