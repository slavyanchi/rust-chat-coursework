use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6000")?;
    println!("Підключено до сервера!");

    let mut stream_clone = stream.try_clone()?;

    thread::spawn(move || {
        let mut reader = BufReader::new(stream_clone);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, 
                Ok(_) => {
                    print!("Повідомлення: {}", line);
                }
                Err(e) => {
                    println!("Помилка зчитування: {}", e);
                    break;
                }
            }
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        stream.write_all(line.as_bytes())?;
        stream.write_all(b"\n")?;
    }
    Ok(())
}
