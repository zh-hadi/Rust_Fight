#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area( &self)->u32{
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 30,
        height: 40,
    };

    println!("The area is Squar Braket {}", rec1.area());

    dbg!(rec1);
}
