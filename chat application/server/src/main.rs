#[allow(warnings)]
#[allow(unused_variables)]

use std::{
        net::{TcpListener, TcpStream},
        io::{prelude::*, BufReader}};
use std::thread;
mod userdata;
use std::str::from_utf8;
use std::sync::Arc;
use std::sync::mpsc;
use std::io;


struct StreamWithInfo {
    name: String,
    stream: TcpStream,
}

fn main() {

    let listener=TcpListener::bind("127.0.0.1:8000").unwrap();
    let (tx, rx) = mpsc::channel();
    let mut user_data=userdata::User::new();
    let user1=userdata::UserInfo{
        name:String::from("shrinivas"),
        password:String::from("shri")
    };
    let user2=userdata::UserInfo{
        name:String::from("omkar"),
        password:String::from("omkar6612")
    };
    user_data.user_data.push(user1);
    user_data.user_data.push(user2);

    thread::spawn(move||{
        handle_send(rx);
    });
    let userdata1=Arc::new(user_data);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        let data = Arc::clone(&userdata1);
        let tx1 = tx.clone();
        thread::spawn(move||{
            handle_recv(stream,data,tx1);
        });
    }
}


fn handle_recv(mut stream:TcpStream,userdata:Arc<userdata::User>,tx:mpsc::Sender<StreamWithInfo>){
    let mut data:Vec<u8> = vec![0; 100];
    let mut flag=0;
    let mut name=String::new();
    match stream.read(&mut data){
        Ok(size) => {
            if size!=0{
                let text=from_utf8(&data).unwrap();
                let mut parts: Vec<&str> = text.split(":").collect();
                parts[0]=parts[0].trim();
                parts[1]=parts[1].trim();
                for user in &userdata.user_data{
                    println!("User {}:{}",user.name,user.password);
                    let value1=user.name==parts[0].to_string();
                    let value2=user.password==parts[1].to_string();
                    println!("bool {value1}:{value2}");
                    if user.name==parts[0].to_string() && user.password==parts[1].to_string(){
                        let send=stream.try_clone().expect("could not clone");
                        let streaminfo=StreamWithInfo{
                            name:parts[0].to_string(),
                            stream:send
                        };
                        name=user.name.clone();
                        tx.send(streaminfo).unwrap();
                        println!("User {} is Authenticated successfully",user.name);
                        flag=1;
                        let aut=String::from("authenticated:auth");
                        match stream.write_all(aut.as_bytes()){
                            Ok(_value)=>{
                                println!("!!!!Sent Authentication info!!!!")
                            },
                            Err(_)=> println!("!!!!Error Sending Authentication Msg!!!!")
                        };
                        break;
                    }
                }
                if flag==0{
                    println!("User {}:{} Authentication failed",parts[0],parts[1]);
                }
            }
        }
        ,
        Err(_e) => {
            println!("Could not recieve the Authentication Credentials");

        }
    }
    let mut data1:Vec<u8> = vec![0; 100];
    if flag==1{

        loop{
            match stream.read(&mut data1){
                Ok(size) => {
                    if size!=0{
                        let text=from_utf8(&data1).unwrap();
                        println!("From {}:{}",name,text.to_string());
                    }
                }
                ,
                Err(_e) => {
                    println!("Could not recieve the Msg");
       
                }
            }
        }
    }
}

fn handle_send(rx:mpsc::Receiver<StreamWithInfo>){
    let mut v:Vec<StreamWithInfo>=Vec::new();
    
    loop{
        let mut send=String::new();
        match rx.try_recv(){
            Ok(value)=>{
                let name=value.name.clone();
                v.push(value);
                println!("{} is available now", name);
            },
            Err(_)=>()
        };
        
            io::stdin()
                .read_line(&mut send)
                .expect("Failed to read line");
        if send!="refresh"{
            let parts: Vec<&str> = send.split(':').collect();
            for user in &mut v{
                if user.name==parts[0]{
                    match user.stream.write_all(parts[1].as_bytes()){
                        Ok(_value)=>{
                            println!("!!!!Sent to {}!!!!",user.name);
                        },
                        Err(_)=> println!("!!!!Error Sending Msg to {}!!!!",user.name)
                    };
                }
            }
        }
    }
}