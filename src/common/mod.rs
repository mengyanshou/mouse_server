use std::net::{TcpListener, TcpStream};
use std::io::Read;
use enigo::{Enigo, MouseControllable, MouseButton};



#[allow(clippy::similar_names)]
fn handle_raw_connection(mut stream: TcpStream) {
    let mut buf = [0; 128];
    loop {
        // 读取内容
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            println!("ok");
            break;
        }
        let message_type = buf[0];
        match message_type{
            0=>{
                let key = buf[1];
                let key = key as char;
                // Enigo::new().key_click(key);
            }
            1=>{
                let x = buf[1] as i8;
                let y = buf[2] as i8;
                println!("x:{},y:{}",x,y);
                let (size_x, size_y) = Enigo::new().main_display_size();
                let (current_x, current_y) = Enigo::new().mouse_location();
                let mut new_x = current_x + x as i32;
                let mut new_y = current_y + y as i32;
                if new_y > size_y -1{
                    new_y = size_y -1;
                }
                if new_x > size_x -1 {
                    new_x = size_x -1;
                }
                if new_y < 0{
                    new_y = 0
                }
                println!("current_x:{},current_y:{}",current_x,current_y);
                Enigo::new().mouse_move_to(new_x, new_y);
            }
            2=>{
                Enigo::new().mouse_click(MouseButton::Left);
            }
            3=>{
                Enigo::new().mouse_click(MouseButton::Right);
            }
            4=>{
                let len=buf[1] as i8;
                println!("len:{}",len);
                Enigo::new().mouse_scroll_y(len as i32);
            }
            _ => {
                println!("Other Message received");
            }
        }
        
        println!("{}",message_type);
        // 输出读取到的内容
        // println!("read {} bytes: {:?}", len, str::from_utf8(&buf[..len]));
    }
}
pub fn launch_ws_server() {
    let listener = TcpListener::bind("0.0.0.0:26541").unwrap();
    println!("TcpListener was created");
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New connection was made from {addr:?}");
                std::thread::spawn(move || handle_raw_connection(stream));
            }
            Err(e) => {
                println!("Connection failed: {e:?}");
            }
        }
    }
    // match listener.accept() {
    //     Ok((stream, addr)) => {
    //         println!("New connection was made from {addr:?}");
    //         std::thread::spawn(move || handle_raw_connection(stream));
    //     }
    //     Err(e) => {
    //         println!("Connection failed: {e:?}");
    //     }
    // }
}
