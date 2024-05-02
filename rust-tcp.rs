use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    // buffer for reading data from client
    let mut buffer = [0; 1024];
    // reads data from the stream and stores in data in the buffer 
    stream.read(&mut buffer).expect("Failed to read from client!");
    // converts data in buffer to UTF8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Recieved request: {}", request);
    let responce = "Hello, Client!".as_bytes();
    stream.write(responce).expect("Failed to write responce!");
}

//entry point
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to established connection: {}", e);

            }
        }
    }
}
