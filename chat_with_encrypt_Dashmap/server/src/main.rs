#[allow(warnings)]
#[allow(unused_variables)]


use std::{
        net::{TcpListener, TcpStream},
        io::{prelude::*}};
use std::thread;
use std::str::from_utf8;
use std::sync::Arc;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io;
use dashmap::DashMap;
use std::time::Duration;
use sha2::{Sha256, Digest};
use hex;


fn main(){


    let mut file = File::open("/home/shri/Desktop/chat1/server/src/identity.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = Identity::from_pkcs12(&identity, "Shri@1234").unwrap();


    let listener=TcpListener::bind("127.0.0.1:8000").unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);

    

    let user_map:Arc<DashMap<String, String>> = Arc::new(DashMap::new());
    let stream_map:Arc<DashMap<String, TlsStream<TcpStream>>> = Arc::new(DashMap::new());

    let shared_data = Arc::clone(&stream_map);
    let handle_send=thread::spawn(move||{
        handle_send(shared_data);
    });
    for stream in listener.incoming(){
        let stream=stream.unwrap();
        let user_map1 = Arc::clone(&user_map);
        let stream_map1 = Arc::clone(&stream_map);
        let acceptor = acceptor.clone();
        thread::spawn(move||{
            let stream = acceptor.accept(stream).unwrap();
            handle_recv(stream,user_map1,stream_map1);
        });
    }
    handle_send.join().expect("send msg panicked");
}

fn handle_recv(mut stream:TlsStream<TcpStream>,user_map:Arc<DashMap<String,String>>,stream_map:Arc<DashMap<String,TlsStream<TcpStream>>>){
    println!("entered handle recv");
    let (success,user_name)=auth_user(&mut stream,&user_map,&stream_map);
    if success{
        match stream.write_all("Authenticated".as_bytes()){
            Ok(_value)=>println!("{} Authentication msg sent successfully",user_name),
            Err(_)=>println!("{} failed to send the msg",user_name)
        }
        stream_map.insert(user_name.clone(),stream);
        loop{
            let mut data = vec![10; 100];
            if let Some(mut stream)=stream_map.get_mut(&user_name){
                let stream=&mut *stream;            
                match stream.read(&mut data){
                    Ok(size) => {
                        if size!=0{
                            let txt=from_utf8(&data).unwrap();
                            let text=txt.trim();
                            println!("{}:{}",user_name,text.trim());
                            if "Quit"==text{
                                let _=stream_map.remove(&user_name).unwrap();
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
        }
    }else{
        match stream.write_all("Not Authenticated".as_bytes()){
            Ok(_value)=>println!("{} not Authentication msg sent successfully",user_name),
            Err(_)=>println!("{} failed to send the msg of not authenticated",user_name)
        }
        println!("user {user_name} not authenticated");
    }
}
fn handle_send(user_stream:Arc<DashMap<String,TlsStream<TcpStream>>>){
    let mut input=String::new();
    loop{
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parts: Vec<&str> = input.split(":").collect();
        if let Some(mut stream)=user_stream.get_mut(parts[0]){
            println!("entered the sending part");
            let stream=&mut *stream;
            match stream.write_all(parts[1].as_bytes()){
                Ok(_value)=>println!("sent to {}",parts[0]),
                Err(_)=>println!("couldn't send the msg")
            }
        }else{
            println!("no user name {}",parts[0]);
        }
        input.clear();
    }
}

fn auth_user(stream:&mut TlsStream<TcpStream>,user_map:&Arc<DashMap<String,String>>,stream_map:&Arc<DashMap<String,TlsStream<TcpStream>>>)->(bool,String){
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
fn register(stream:&mut TlsStream<TcpStream>,user_map:&Arc<DashMap<String,String>>){
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
                if let Some(_value) = user_map.get(&user_name) {
                    match stream.write_all("user name already present".as_bytes()){
                        Ok(_value)=> println!("user already present"),
                        Err(_)=>println!("Error sending msg user already present")
                    };
                }else{
                    let username=user_name.clone();
                    let mut hasher = Sha256::new();
                    hasher.update(user_password.as_bytes());
                    let result = hasher.finalize();
                    let bytes = hex::encode(result);
                    println!("Hash:{bytes}");
                    user_map.insert(user_name,bytes);
                    
                    match stream.write_all("Registered".as_bytes()){
                        Ok(_value)=> println!("{username} registered succesfully"),
                        Err(_)=>println!("Error sending msg registered")
                    };
                    let duration = Duration::from_secs(1);
                    thread::sleep(duration);
                }
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
fn login(stream:&mut TlsStream<TcpStream>,user_map:&Arc<DashMap<String,String>>,stream_map:&Arc<DashMap<String,TlsStream<TcpStream>>>)->(bool,String){
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
            if let Some(value) = user_map.get(&user_name) {
                if let None=stream_map.get(&user_name){
                    if value.as_ref()==user_password{
                        value1=true;
                    }
                }
            }
            return (value1,user_name);
        },
        Err(_)=>return (false,String::from("rand"))
    }
}