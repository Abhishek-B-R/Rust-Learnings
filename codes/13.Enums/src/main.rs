enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String)
}

impl IpAddr{
    fn display(&self){
        match self{
            IpAddr::V4(a,b,c,d) => println!("{}.{}.{}.{}",a,b,c,d),
            IpAddr::V6(addr) => println!("{}",addr),
        }
    }
}

fn main() {
    // let four = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 3, 2, 1)); this is how its done with standard library
    let four = IpAddr::V4(127,0,0,1);
    let six = IpAddr::V6(String::from("::1"));
    let none : Option<i32> = None;

    four.display();
    six.display();
    println!("{:?}",none);
}
