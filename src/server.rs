use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Клієнт відключився: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                println!("Отримано: {} від {}", msg, stream.peer_addr().unwrap());

                let mut clients_lock = clients.lock().unwrap();
                for client in clients_lock.iter_mut() {
                    if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                        let _ = client.write_all(msg.as_bytes());
                    }
                }
            }
            Err(e) => {
                println!("Помилка зчитування: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6000")?;
    let clients = Arc::new(Mutex::new(Vec::new()));
    println!("Сервер запущено на 127.0.0.1:6000");

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Новий клієнт: {}", stream.peer_addr()?);

        let clients_clone = Arc::clone(&clients);
        clients_clone.lock().unwrap().push(stream.try_clone()?);

        thread::spawn(move || {
            handle_client(stream, clients_clone);
        });
    }
    Ok(())
}
