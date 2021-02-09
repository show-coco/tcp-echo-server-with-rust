use std::{
  error,
  io::{self, prelude::*},
  net, str, thread,
};

fn main() -> Result<(), Box<dyn error::Error>>{
  // ソケットの生成とローカルアドレスへの紐付け
  let listener = net::TcpListener::bind("127.0.0.1:50000")?;
  loop {
    let (stream, _) = listener.accept()?; // 接続の待ち受け
    thread::spawn(move || {
      handler(stream).unwrap();
    });
  }
}

// クライアントが接続しにきたときの処理
fn handler(mut stream: net::TcpStream) -> Result<(), Box<dyn error::Error>> {
  println!("incoming connection from {}", stream.peer_addr()?);
  loop {
    let mut reader = io::BufReader::new(&stream);
    let mut buf = vec![];
    match reader.read_until(b'\n', &mut buf)? {
      0 => {
        println!("connection closed");
        return Ok(())
      },
      n => {
        print!("{}", str::from_utf8(&buf[..n])?);
        stream.write_all(&buf[..n])?;
      }
    }
  }
}