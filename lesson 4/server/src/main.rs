use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();  //TcpListener 用于监听 TCP 连接，绑定地址 127.0.0.1:7878

    for stream in listener.incoming() {
        match stream { //错误处理匹配
            Ok(stream) =>{                            //连接成功
                println!("Connection established!");  //打印“连接成功”
                handle_connection(stream); //调用handle_connection 函数并向其传递 stream
            }
            Err(e) => {                    //连接失败
                println!("Error: {}", e); //打印“错误提示”
            }
        }
        
        
    }
}
fn handle_connection(mut stream: TcpStream) {  //处理stream的函数
    let mut buffer = [0; 512];                 //声明一个 buffer 来存放读取到的数据。这里创建了一个 512 字节的缓冲区

    stream.read(&mut buffer).unwrap();    //从 TcpStream 中读取字节并放入缓冲区中。
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); //将缓冲区中的字节转换为字符串并打印出来
    stream.write(&buffer).unwrap();  //stream 的 write 方法将buffer发送给Tcpstream。
    stream.flush().unwrap();  //flush 会等待并阻塞程序执行直到所有字节都被写入连接中
}