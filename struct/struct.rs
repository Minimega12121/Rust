struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
   let mut user1 = User{
    email : String::from("arhcit@hmail.com"),
    username: String::from("Archit"),
    active: true,
    sign_in_count: 1
   } ;

   let name = user1.username;

   user1.username = String::from("Abhinav");

   let user2 = build_user(String::from("adfasfasd"),String::from("asdfasdfasdf"));

   let user3 = User{
    email: String::from("aviral@hotmail.comn"),
    username: String::from("Aviral"),
    ..user2
   };


   let rect = (u,u32)
}

fn build_user(email: String, username:  String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}