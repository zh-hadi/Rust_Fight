// Enum is user define data type 
// Enum is some usefull case rather than struct
#[derive(Debug)]
enum IpAddr { // declear enum 
    Ip4(String),
    Ip6(String),
}

fn main() {
    // enum value declear
    let home_ip = IpAddr::Ip4(String::from("127.0.0.1"));
    dbg!(home_ip);

}
