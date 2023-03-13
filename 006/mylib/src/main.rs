use core::time;
use mylib::ThreadPool;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("----------------------------------------------------");

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();

        // 单线程的方式
        // handle_client(stream);

        // 多线程的方式
        // let handle = thread::spawn(move || {
        //     handle_client(stream);
        // });
        // thread_vec.push(handle);

        // 线程池的方式
        pool.execute(|| {
            handle_client(stream);
        });
    }

    // for handle in thread_vec {
    //     handle.join().unwrap();
    // }
    Ok(())
}
