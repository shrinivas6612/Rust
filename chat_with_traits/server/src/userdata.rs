use std::net:: TcpStream;
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
    pub fn check(&self,user_name:&str,user_password:&str)->bool{
        for v in &self.user_data{
            if v.name==user_name  && v.password==user_password{
                return true;
            }
        }
        false
    }
    pub fn register(&mut self, user_name:String,user_password:String)->bool{
        for v in &mut self.user_data{
            if v.name==user_name{
                return false;
            }
        }
        let _=&mut self.user_data.push(UserInfo{name:user_name,password:user_password});
        true
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
    pub fn check(&self,user_name:&str)->bool{
        for v in &self.user_conn{
            if v.name==user_name{
                return false
            }
        }
        true
    }
    pub fn get(&mut self,user_name:&str)->Result<&mut TcpStream,String>{
        for v in &mut self.user_conn{
            if v.name==user_name{
                return Ok(&mut v.session)
            }
        }
        Err("No user named {user_name}".to_string())
    }
    pub fn insert(&mut self, user_name:String, user_str:TcpStream)->Result<bool,String>{
        let _=&mut self.user_conn.push(UserStream{name:user_name,session:user_str});
        Ok(true)
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