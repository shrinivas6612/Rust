use std::net:: TcpStream;

pub trait Verify{
    type Item;
    fn check(&self,user_name:&str)->bool;
    fn insert(&mut self,user_name:String,item:Self::Item)->Result<String,String>;
}
struct UserInfo{
    pub name:String,
    pub password:String
}

pub struct User{
    user_data :Vec<UserInfo>
}

impl User{
    pub fn new()->Self{
        Self{
            user_data:Vec::new()
        }
    }
    pub fn login(&self,user_name:&str,user_password:&str)->bool{
        for v in &self.user_data{
            if v.name==user_name  && v.password==user_password{
                return true;
            }
        }
        false
    }
}
impl Verify for User{
    type Item=String;
    fn check(&self,user_name:&str)->bool{
        for v in &self.user_data{
            if v.name==user_name{
                return true;
            }
        }
        false
    }
    fn insert(&mut self, user_name:String,user_password:String)->Result<String,String>{
        if self.check(&user_name){
            return Err("Username already present".to_string());
        }
        let name=user_name.clone();
        let _=&mut self.user_data.push(UserInfo{name:user_name,password:user_password});
        Ok(format!("{} Registered successfully",name))
    }
}


struct UserStream{
    name:String,
    session:TcpStream
}

pub struct UserConnections{
    user_conn:Vec<UserStream>
}


impl UserConnections{
    pub fn new()->Self{
        Self{
            user_conn:Vec::new()
        }
    }
    pub fn get(&mut self,user_name:&str)->Result<&mut TcpStream,String>{
        for v in &mut self.user_conn{
            if v.name==user_name.to_string(){
                return Ok(&mut v.session)
            }
        }
        Err(format!("No user named {}",user_name))
    }
    pub fn remove(&mut self, user_name:&str)->bool{
        let mut i = 0;
        while i < self.user_conn.len() {
            if self.user_conn[i].name  == user_name {
                let _=&mut self.user_conn.remove(i);
                return true 
            }
            i=i+1;
        }
        false
    }
}

impl Verify for UserConnections{
    type Item=TcpStream;
    fn check(&self,user_name:&str)->bool{
        for v in &self.user_conn{
            if v.name==user_name{
                return true
            }
        }
        false
    }
    fn insert(&mut self, user_name:String, user_str:Self::Item)->Result<String,String>{
        if self.check(&user_name){
            return Err("User already loggedIn".to_string());
        }
        let name=user_name.clone();
        let _=&mut self.user_conn.push(UserStream{name:user_name,session:user_str});
        Ok(format!("{} inserted successfully",name))
    }

}