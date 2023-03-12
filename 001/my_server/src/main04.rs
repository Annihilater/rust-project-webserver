use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    println!("----------------------------------------------------");

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        // 返回 main.html 内容
        let content = fs::read_to_string("main.html").unwrap();
        let response = format!("HTTP/1.1 200 ok\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 返回 404.html 内容
        let content = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 ok\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    };
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
