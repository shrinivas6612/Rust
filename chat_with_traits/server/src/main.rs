#[allow(warnings)]
#[allow(unused_variables)]


use std::{
        net::{TcpListener, TcpStream},
        io::{prelude::*}};
use std::thread;
use std::str::from_utf8;
use std::sync::{Arc,RwLock};
use std::io;
use std::time::Duration;
use sha2::{Sha256, Digest};
use hex;
mod userdata;
use crate::userdata::Verify;



fn main(){
    let listener=TcpListener::bind("127.0.0.1:8000").unwrap();
    

    let user_map=userdata::User::new();
    let user_map=Arc::new(RwLock::new(user_map));
    let user_stream=userdata::UserConnections::new();
    let stream_map=Arc::new(RwLock::new(user_stream));

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

fn handle_recv(mut stream:TcpStream,user_map:Arc<RwLock<userdata::User>>,stream_map:Arc<RwLock<userdata::UserConnections>>){
    let (success,user_name)=auth_user(&mut stream,&user_map,&stream_map);
    if success{
        match stream.write_all("Authenticated".as_bytes()){
            Ok(_value)=>println!("{} Authentication msg sent successfully",user_name),
            Err(_)=>println!("{} failed to send the msg",user_name)
        }
        {
            let mut user_stream=stream_map.write().unwrap();
            let result=user_stream.insert(user_name.clone(),stream.try_clone().expect("Failed to clone stream"));
            match result{
                Ok(value)=>println!("{value}"),
                Err(err)=>println!("{err}")
            }
        }
        loop{
            let mut data = vec![10; 100];            
            match stream.read(&mut data){
                Ok(size) => {
                    if size!=0{
                        let txt=from_utf8(&data).unwrap();
                        let text=txt.trim();
                        println!("{}:{}",user_name,text.trim());
                        if "Quit"==text{
                            {
                                let mut user_stream=stream_map.write().unwrap();
                                user_stream.remove(&user_name);
                    
                            }
                            println!("{user_name} logged out");
                            break;
                        }
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
        println!("user {user_name} not authenticated");
    }
}
fn handle_send(stream_map:Arc<RwLock<userdata::UserConnections>>){
    let mut input=String::new();
    loop{
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parts: Vec<&str> = input.split(":").collect();
        {
            let mut user_stream=stream_map.write().unwrap();
            let res=user_stream.get(&parts[0]);
            match res{
                Ok(stream)=>{
                    match stream.write_all(parts[1].as_bytes()){
                        Ok(_value)=>println!("sent to {}",parts[0]),
                        Err(_)=>println!("couldn't send the msg")
                    }
                },
                Err(err)=>{
                    println!("{err}");
                }
            }
            
        }
        input.clear();
    }
}

fn auth_user(mut stream:&TcpStream,user_map:&Arc<RwLock<userdata::User>>,stream_map:&Arc<RwLock<userdata::UserConnections>>)->(bool,String){
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
                                2=> return login(stream,user_map,stream_map),
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
fn register(mut stream:&TcpStream,user_map:&Arc<RwLock<userdata::User>>){
    let mut user_name=String::new();
    let mut user_password=String::new();
    match stream.write_all("Enter user name to register".as_bytes()){
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
            match stream.write_all("Enter user password to register".as_bytes()){
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
            if(user_name!="" || user_name!=" ") && (user_password!="" || user_password!=" "){
                let mut hasher = Sha256::new();
                hasher.update(user_password.as_bytes());
                let result = hasher.finalize();
                let bytes = hex::encode(result);
                println!("Hash:{bytes}");
                let mut user_check=user_map.write().unwrap();
                let _reg_success=user_check.insert(user_name,bytes);
                match _reg_success{
                    Ok(value1)=>{
                        match stream.write_all("Registered".as_bytes()){
                            Ok(_value)=> println!("registerion msg sent succesfully"),
                            Err(_)=>println!("Error sending msg registered")
                        };
                        println!("{value1}");
                    }
                    Err(err)=>{
                        match stream.write_all("user name already present".as_bytes()){
                            Ok(_value)=> println!("user already present sent successfully"),
                            Err(_)=>println!("Error sending msg user already present")
                        };
                        println!("{err}");
                    }
                }
                let duration = Duration::from_secs(1);
                thread::sleep(duration);
            }else{
                match stream.write_all("user name or password is not valid".as_bytes()){
                    Ok(_value)=> println!("user name or password is not valid, not registered"),
                    Err(_)=>println!("Error sending msg user already present")
                };
            }
        },
        Err(_)=>println!("Couldn't send enter user name for register")
    }
}
fn login(mut stream:&TcpStream,user_map:&Arc<RwLock<userdata::User>>,stream_map:&Arc<RwLock<userdata::UserConnections>>)->(bool,String){
    println!("login");
    let mut user_name=String::new();
    let mut user_password=String::new();
    match stream.write_all("Enter user name to login".as_bytes()){
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
            match stream.write_all("Enter user password to login".as_bytes()){
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
            let user_check=user_map.read().unwrap();
            if user_check.login(&user_name,&user_password) {
                let user_stream=stream_map.read().unwrap();
                if user_stream.check(&user_name)==false{
                        value1=true;
                }
            }
            return (value1,user_name);
        },
        Err(_)=>return (false,String::from("rand"))
    }
}