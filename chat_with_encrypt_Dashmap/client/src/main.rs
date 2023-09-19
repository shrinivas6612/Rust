use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::sync::{Arc,Mutex};
use std::io;
use std::str::from_utf8;
use std::sync::mpsc;
use sha2::{Sha256, Digest};
use native_tls::{TlsConnector,TlsStream};
use hex;
//use std::time::Duration;


fn main(){
    //let connector = TlsConnector::new().unwrap();
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true) // Skip certificate verification
        .danger_accept_invalid_hostnames(true) // Skip hostname verification
        .build().unwrap();
    if let Ok(stream) = TcpStream::connect("localhost:8000") {
        if let Ok(stream) = connector.connect("localhost", stream){
            let (tx, rx) = mpsc::channel::<bool>();

            let mut stream=Arc::new(Mutex::new(stream));
            let send_stream=Arc::clone(&mut stream);
            let recv_stream=Arc::clone(&mut stream);

            let send_fun = thread::spawn(move || {
                send_data(send_stream,tx);
            });

            let recv_fun = thread::spawn(move || {
                recv_data(recv_stream,rx);
            });

            send_fun.join().expect("Send thread panicked");
            recv_fun.join().expect("Receive thread panicked");
        }else{
            println!("Couldnot upgrade to tls connection");
        }
    }else{
        println!("Couldn't connect to server");
    }
}

fn send_data(mut stream:Arc<Mutex<TlsStream<TcpStream>>>,tx:mpsc::Sender<bool>){
    if auth_user(&mut stream){
        println!("Authenticated successfully!!!");
        println!("You can start sending msgs");
        let mut guess = String::new();
        tx.send(true).expect("Send error");
        
        loop{
            let mut stream=stream.lock().unwrap();
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
            if "Quit"==guess.trim(){
                let _value=stream.shutdown();
                println!("Quiting");
                break;
            }
            guess.clear();
        }
    }else{
        println!("Authentication failed");
        tx.send(false).expect("Send error");
    }
}

fn recv_data(stream:Arc<Mutex<TlsStream<TcpStream>>>,rx:mpsc::Receiver<bool>){
    let auth = rx.recv().expect("Receive error");
    if auth==true{
        loop{
            let mut stream=stream.lock().unwrap();
            let mut data = vec![10; 100];       
            match stream.read(&mut data){
                Ok(size) => {
                    if size!=0{
                        let txt=from_utf8(&data).unwrap();
                        let text=txt.trim();
                        println!("server:{}",text.trim());
                    }                    
                }
                ,
                Err(_e) => {
                    println!("Could not recieve the msg");
                    break;
                }
            }
        }
    }
}

fn auth_user(stream:&mut Arc<Mutex<TlsStream<TcpStream>>>)->bool{
    println!("Entered auth user");
    
    loop{
        let mut stream=stream.lock().unwrap();
        let mut data = vec![10; 100];
        match stream.read(&mut data){
            Ok(size)=>{
                if size!=0{
                    let txt=from_utf8(&data).unwrap();
                    let text=txt.trim();
                    println!("{text}");
                    let choice="Choose 1.Register 2.Login 3.Quit";
                    let luser="Enter user name to login";
                    let lpass="Enter user password to login";
                    let ruser="Enter user name to register";
                    let rpass="Enter user password to register";
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
                        s if s==luser || s==lpass =>{
                            let mut sel  = String::new();
                            io::stdin()
                                .read_line(&mut sel)
                                .expect("Failed to read line");
                            if s==lpass{                                
                                let mut hasher = Sha256::new();
                                hasher.update(sel.trim().as_bytes());
                                let result = hasher.finalize();
                                sel = hex::encode(result);
                                println!("Hash:{sel}");
                            }
                            match stream.write_all(sel.as_bytes()){
                                Ok(_value)=>{
                                    println!("!!!!Sent user information!!!!")
                                },
                                Err(_)=> println!("!!!!Error Sending Msg!!!!")
                            };
                        },
                        s if s==ruser || s==rpass =>{
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
                            return false
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