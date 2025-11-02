struct User{
    name : String,
    age : u64,
    active : bool,
    email : String,
}
fn main() {
    let mut user1 = User {
        name : String::from("Abhishek"),
        age : 20,
        active : true,
        email : String::from("abhishekbr989@gmail.com"),
    };

    user1.name = String::from("Abhishek BR");

    println!("{}",user1.name);
    println!("{}",user1.age);
    println!("{}",user1.active);
    println!("{}",user1.email);


    // let _user2 = User {
    //     name: user1.name,
    //     age: user1.age,
    //     active: user1.active,
    //     email: String::from("another@example.com"),
    // };

    let _user3 = User {
        email: String::from("anothersecond@example.com"),
        ..user1
    };

}
