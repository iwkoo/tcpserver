use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;//读取流数据
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write(&buf[..bytes_read])?; //讲收到的数据应答给客户端
        println!("服务器收到：{} ，并应答！", String::from_utf8_lossy(&buf));
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

//主函数
fn main() -> std::io::Result<()> {
    println!("服务器启动!监听 127.0.0.1:8081 ");
    let listener = TcpListener::bind("127.0.0.1:8081")?; //监听本地端口
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();//创建线程
    //循环读取客户端传来的信息
    for stream in listener.incoming() {
        let stream = stream.expect("failed!");//读取流数据
        let handle = thread::spawn(move || {
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        });//在线程中处理流数据

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}