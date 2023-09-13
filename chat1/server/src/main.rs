#[allow(warnings)]
#[allow(unused_variables)]

use std::{
        net::{TcpListener, TcpStream},
        io::{prelude::*, BufReader}};
use std::thread;
use std::str::from_utf8;
use std::sync::Arc;
//use std::sync::mpsc;
use std::io;
use dashmap::DashMap;
use std::time::Duration;


fn main(){
    let listener=TcpListener::bind("127.0.0.1:8000").unwrap();

    let user_map:Arc<DashMap<String, String>> = Arc::new(DashMap::new());
    let stream_map:Arc<DashMap<String, TcpStream>> = Arc::new(DashMap::new());

    let shared_data = Arc::clone(&stream_map);
    let handle_send=thread::spawn(move||{
        handle_send(shared_data);
    });
    for stream in listener.incoming(){
        let stream=stream.unwrap();
        let user_map1 = Arc::clone(&user_map);
        let stream_map1 = Arc::clone(&stream_map);
        thread::spawn(move||{
            handle_recv(stream,user_map1,stream_map1);
        });
    }
    handle_send.join().expect("send msg panicked");
}

fn handle_recv(mut stream:TcpStream,user_map:Arc<DashMap<String,String>>,stream_map:Arc<DashMap<String,TcpStream>>){
    println!("entered handle recv");
    let (success,user_name)=auth_user(&mut stream,&user_map);
    if success{
        match stream.write_all("Authenticated".as_bytes()){
            Ok(_value)=>println!("{} Authentication msg sent successfully",user_name),
            Err(_)=>println!("{} failed to send the msg",user_name)
        }
        stream_map.insert(user_name.clone(),stream.try_clone().expect("Failed to clone stream"));
        loop{
            let mut data = vec![10; 100];            
            match stream.read(&mut data){
                Ok(size) => {
                    if size!=0{
                        let txt=from_utf8(&data).unwrap();
                        let text=txt.trim();
                        println!("{}:{}",user_name,text.trim());
                    }                    
                }
                ,
                Err(_e) => {
                    println!("Could not recieve the error");

                }
            }           
        }
    }else{
        match stream.write_all("Not Authenticated".as_bytes()){
            Ok(_value)=>println!("{} not Authentication msg sent successfully",user_name),
            Err(_)=>println!("{} failed to send the msg of not authenticated",user_name)
        }
        println!("user not authenticated");
    }
}
fn handle_send(user_stream:Arc<DashMap<String,TcpStream>>){
    loop{
        let mut input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parts: Vec<&str> = input.split(":").collect();
        if let Some(stream)=user_stream.get(parts[0]){
            let mut stream=&*stream;
            match stream.write_all(parts[1].as_bytes()){
                Ok(_value)=>println!("sent to {}",parts[0]),
                Err(_)=>println!("couldn't send the msg")
            }
        }else{
            println!("no user name {}",parts[0]);
        }
    }
}

fn auth_user(mut stream:&TcpStream,user_map:&Arc<DashMap<String,String>>)->(bool,String){
    // let duration = Duration::from_secs(5);
    // thread::sleep(duration);
    loop{
        match stream.write_all("Choose 1.Register 2.Login 3.Quit".as_bytes()){
            Ok(_value)=>{
                println!("!!!!Sent!!!!");
                let mut data = vec![10; 100];
                match stream.read(&mut data){
                    Ok(size)=>{
                        if size!=0{
                            let txt=from_utf8(&data).unwrap();
                            let text=txt.trim();
                            let value: u32 = match text.trim().parse() {
                                Ok(num) => num,
                                Err(_) => 0,
                            };
                            match value{
                                1=> register(stream,user_map),
                                2=> return login(stream,user_map),
                                3=> return (false,String::from("rand")),
                                _=> ()
                            };
                        }
                    },
                    Err(_)=>println!("couldn't read user input from client")
                }
            },
            Err(_)=> {
                println!("!!!!Error Sending Msg!!!!");
                return (false,String::from("rand"))
            }
        }
    }
}
fn register(mut stream:&TcpStream,user_map:&Arc<DashMap<String,String>>){
    let mut user_name=String::new();
    let mut user_password=String::new();
    match stream.write_all("Enter user name".as_bytes()){
        Ok(_value)=>{
            let mut data = vec![10; 100];
            match stream.read(&mut data){
                Ok(size)=>{
                    if size!=0{
                        let txt=from_utf8(&data).unwrap();
                        let text=txt.trim();
                        user_name=text.to_string();
                    }
                },
                Err(_)=>println!("Couldn't read user_name")
            }
            match stream.write_all("Enter user password".as_bytes()){
                Ok(_value)=>{
                    let mut data = vec![10; 100];
                    match stream.read(&mut data){
                        Ok(size)=>{
                            if size!=0{
                                let txt=from_utf8(&data).unwrap();
                                let text=txt.trim();
                                user_password=text.to_string();
                            }
                        },
                        Err(_)=>println!("Couldn't read user_password from client")
                    }
                },
                Err(_)=>println!("Couldn't send enter user password for register")
            }
            if let Some(_value) = user_map.get(&user_name) {
                match stream.write_all("user name already present".as_bytes()){
                    Ok(_value)=> println!("user already present"),
                    Err(_)=>println!("Error sending msg user already present")
                };
            }else{
                let username=user_name.clone();
                user_map.insert(user_name,user_password);
                match stream.write_all("Registered".as_bytes()){
                    Ok(_value)=> println!("{username} registered succesfully"),
                    Err(_)=>println!("Error sending msg registered")
                };
                let duration = Duration::from_secs(1);
                thread::sleep(duration);
            }
        },
        Err(_)=>println!("Couldn't send enter user name for register")
    }
}
fn login(mut stream:&TcpStream,user_map:&Arc<DashMap<String,String>>)->(bool,String){
    println!("login");
    let mut user_name=String::new();
    let mut user_password=String::new();
    match stream.write_all("Enter user name".as_bytes()){
        Ok(_value)=>{
            let mut data = vec![10; 100];
            match stream.read(&mut data){
                Ok(size)=>{
                    if size!=0{
                        let txt=from_utf8(&data).unwrap();
                        let text=txt.trim();
                        user_name=text.to_string();
                    }
                },
                Err(_)=>println!("couldn't read user name from client for loging")
            }
            match stream.write_all("Enter user password".as_bytes()){
                Ok(_value)=>{
                    let mut data = vec![10; 100];
                    match stream.read(&mut data){
                        Ok(size)=>{
                            if size!=0{
                                let txt=from_utf8(&data).unwrap();
                                let text=txt.trim();
                                user_password=text.to_string();
                            }
                        },
                        Err(_)=>println!("couldn't rad user password to client to login")
                    }
                },
                Err(_)=>println!("couldn't send enter user password to client to login")
            }
            let mut value1=false;
            if let Some(value) = user_map.get(&user_name) {
                if value.as_ref()==user_password{
                    value1=true;
                }
            }
            return (value1,user_name);
        },
        Err(_)=>return (false,String::from("rand"))
    }
}