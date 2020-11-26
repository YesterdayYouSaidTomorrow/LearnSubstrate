use std::net::{TcpStream};
use std::io::{self, Read, Write};
use std::str;

fn main() {
    match TcpStream::connect("localhost:7878") {            //连接服务器
        Ok(mut stream) => {                                 //连接成功
            println!("Successfully connected to server in port 7878"); //打印“连接成功”

            let mut msg = String::new();  //声明一个变量msg
            io::stdin()
            .read_line(&mut msg)           //给msg赋值
            .expect("Failed to read from stdin"); //如果输入失败，则抛出异常

            stream.write(msg.as_bytes()).unwrap();  //msg写入stream

            let mut data = [0 as u8; 6]; //  //声明一个 data 来存放读取到的数据。
            match stream.read_exact(&mut data) {
                Ok(_) => {                        //读取成功
                        println!("Reply is ok!"); 
                        println!("{}", 
                        str::from_utf8(&data).unwrap());  //打印读取的字符串
                },
                Err(e) => {                       //读取失败
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {                        //连接失败
            println!("Failed to connect: {}", e);  //打印连接失败
        }
    }
    println!("Terminated.");                //连接中止
}