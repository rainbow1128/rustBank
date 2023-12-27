use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Conncection Establised");
            }
            Err(e) => {
                eprintln!("Error while listening to the server :{}", e)
            }
        }
    }
}
