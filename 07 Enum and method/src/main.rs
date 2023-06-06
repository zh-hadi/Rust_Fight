// Enum is user define data type 
// Enum is some usefull case rather than struct
#[derive(Debug)]
enum IpAddr { // declear enum 
    Ip4(String),
    Ip6(String),
}
// method in enum
impl IpAddr {
    fn Ip4_Example(&self){ // we can return value -> return type
        println!("127.0.0.1");
    }
}

#[derive(Debug)]
struct Student {
    Name: String,
    Roll: u32,
}

impl Student{
    fn School(&self){
        println!("Balia Vekutia High School.");
    }
}

// practice rust enum
#[derive(Debug)]
enum Gender {
    Male(String),
    Female(String),
}

// method in struct 

fn main() {
    // enum value declear
    let home_ip = IpAddr::Ip4(String::from("127.0.0.1"));
    dbg!(&home_ip);

    home_ip.Ip4_Example();

    // struct declear
    let s1 = Student {
        Name: String::from("Hadiuzzaman"),
        Roll: 34,
    };

    dbg!(&s1);
    println!("School Name: ");
    s1.School();
    println!("{}", &s1.Name);


    let st1 = Gender::Male(String::from("Hadiuzzaman"));
    dbg!(st1);

    // owner ship is most important thing in rust 
    // 1. reference 
    // 2. borrow 

    // In rust one value is one owner 

}
