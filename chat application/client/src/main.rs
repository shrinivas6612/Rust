use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::io;
use std::str::from_utf8;
use std::sync::mpsc;
use std::time::Duration;


fn main(){
    
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8000") {
        let mut send=stream.try_clone().expect("could not clone");
        let mut recv=stream;
        let (tx, rx) = mpsc::channel();

        let handle=thread::spawn(move||{  
            let mut user_name= String::from("");
            let mut password=String::from("");
            println!("Enter the userName:");
            io::stdin()
                .read_line(&mut user_name)
                .expect("Failed to read line");
            println!("Enter the password");
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read line");
            let user_name=user_name.trim();
            let password=password.trim();
            let response=format!("{}:{}:cred",user_name.to_string(),password.to_string());

            send.write_all(response.as_bytes()).unwrap();
            println!("Credentials sent for authentication");
    
            let received:String= rx.recv().unwrap();
            println!("Received {received}");
            if received.eq("authenticated"){
                println!("Authenticated successfully!!!");
                println!("You can start sending msgs");
                loop{
                    let mut guess = String::from("");
                    io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                    let bytes=guess.into_bytes();
                    match send.write_all(& bytes){
                        Ok(_value)=>{
                            println!("!!!!Sent!!!!")
                        },
                        Err(_)=> println!("!!!!Error Sending Msg!!!!")
                    }
                }
            }else{
                println!("Couldn't Authenticate to the server or Credentials are incorrect")
            }
            
        });
        
        let mut flag=0;
        let _=recv.set_read_timeout(Some(Duration::from_secs(100)));
        let mut data=[0 as u8; 100];
        match recv.read(&mut data){
            Ok(size) => {
                if size!=0{
                    let text=from_utf8(&data).unwrap();
                    let parts: Vec<&str> = text.split(":").map(str::trim).collect();
                    let my_string: String = parts[0].to_string();
                    if my_string.eq("authenticated"){
                        tx.send(my_string).unwrap();
                        flag=1;
                        println!("Authenticated!!!!!")
                    }
                    println!("{}!!!",text);
                }                    
            }
            ,
            Err(_e) => {
                println!("Could not recieve the error");

            }
        }
        let _=recv.set_read_timeout(Some(Duration::from_secs(600)));
        if flag==1{
            loop{ 
                let mut data=[0 as u8; 100];
                match recv.read(&mut data){
                    Ok(size) => {
                        if size!=0{
                            let text=from_utf8(&data).unwrap();
                            println!("{}",text);
                        }                    
                    }
                    ,
                    Err(_e) => {
                        println!("Could not recieve the msgs");
        
                    }
                }
            }
        }else{
            println!("Authentication failed")
        }
        handle.join().unwrap();

    } else {
        println!("Couldn't connect to server...");
    }
}
