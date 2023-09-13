use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
//use std::sync::{Arc, RwLock};
use std::io;
use std::str::from_utf8;
use std::sync::mpsc;
//use std::time::Duration;


fn main(){
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8000") {

        let (tx, rx) = mpsc::channel::<bool>();


        let send_stream=stream.try_clone().expect("could not clone");
        let recv_stream=stream;

        let send_fun = thread::spawn(move || {
            send_data(send_stream,tx);
        });

        let recv_fun = thread::spawn(move || {
            recv_data(recv_stream,rx);
        });

        send_fun.join().expect("Send thread panicked");
        recv_fun.join().expect("Receive thread panicked");
    }else{
        println!("Couldn't connect to server");
    }
}

fn send_data(mut stream:TcpStream,tx:mpsc::Sender<bool>){
    if auth_user(&stream){
        println!("Authenticated successfully!!!");
        println!("You can start sending msgs");
        let mut guess = String::new();
        tx.send(true).expect("Send error");
        loop{
            
            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            let bytes= guess.as_bytes();
            match stream.write_all(& bytes){
                Ok(_value)=>{
                    println!("!!!!Sent!!!!")
                },
                Err(_)=> println!("!!!!Error Sending Msg!!!!")
            }
        }
    }else{
        println!("Authentication failed");
        tx.send(true).expect("Send error");
    }
}

fn recv_data(mut stream:TcpStream,rx:mpsc::Receiver<bool>){
    let auth = rx.recv().expect("Receive error");
    if auth==true{
        loop{     
            let mut data = vec![10; 100];       
            match stream.read(&mut data){
                Ok(size) => {
                    if size!=0{
                        let txt=from_utf8(&data).unwrap();
                        let text=txt.trim();
                        println!("{}",text.trim());
                    }                    
                }
                ,
                Err(_e) => {
                    println!("Could not recieve the error");
    
                }
            }
        }
    }
}

fn auth_user(mut stream:&TcpStream)->bool{
    println!("Entered auth user");
    loop{
        let mut data = vec![10; 100];
        match stream.read(&mut data){
            Ok(size)=>{
                if size!=0{
                    let txt=from_utf8(&data).unwrap();
                    let text=txt.trim();
                    println!("Hello{text}Hello");
                    let choice="Choose 1.Register 2.Login 3.Quit";
                    let euser="Enter user name";
                    let epass="Enter user password";
                    let auth="Authenticated";
                    let regist="Registered";
                    
                    match text{
                        s if choice==s =>{
                            let mut sel  = String::new();
                            io::stdin()
                                .read_line(&mut sel)
                                .expect("Failed to read line");       
                            let sel: u32 = match sel.trim().parse() {
                                Ok(num) => num,
                                Err(_) => 0,
                            };
                            match stream.write_all(sel.to_string().as_bytes()){
                                Ok(_value)=>{
                                    println!("!!!!Sent number!!!!")
                                },
                                Err(_)=> println!("!!!!Error Sending Msg!!!!")
                            }

                        },
                        s if s==auth =>{
                            return true
                        },
                        s if s==regist =>{
                            println!("Registered successfully");
                        },
                        s if s==euser || s==epass =>{
                            let mut sel  = String::new();
                            io::stdin()
                                .read_line(&mut sel)
                                .expect("Failed to read line");
                            match stream.write_all(sel.as_bytes()){
                                Ok(_value)=>{
                                    println!("!!!!Sent user information!!!!")
                                },
                                Err(_)=> println!("!!!!Error Sending Msg!!!!")
                            };
                        },
                        _=>{
                            println!("Error:{text}");
                        },
                    }
                }
            },
            Err(_)=>{
                println!("fail to receive the msg")
            }
        }
    }
}