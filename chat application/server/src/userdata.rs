

    pub struct UserInfo{
        pub name:String,
        pub password:String
    }

    pub struct User{
        pub user_data :Vec<UserInfo>
    }

    impl User{
        pub fn new()->Self{
            Self{
                user_data:Vec::new()
            }
        }
        // pub fn check(&self,user_name:&str,user_password:&str)->bool{
        //     for v in &self.user_data{
        //         if v.name==user_name && v.password==user_password{
        //             return true;
        //         }
        //     }
        //     false
        // }
        // pub fn register(&mut self, user_name:String,user_password:String)->bool{
        //     for v in &self.user_data{
        //         if v.name==user_name{
        //             return false;
        //         }
        //     }
        //     &mut self.user_data.push(UserInfo{name:user_name,password:user_password});
        //     true
        // }
    }