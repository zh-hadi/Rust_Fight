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

impl Rectangle {
    fn width(&self)-> bool{
        self.width > 0
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 30,
        height: 40,
    };

    println!("The area is Squar Braket {}", rec1.area());

    if rec1.width() {
        println!("This rectangle width is nonzero number: {}", rec1.width);
    }

    dbg!(rec1);

    let rec2 = Rectangle{
        width: 0,
        height: 40,
    };

    if rec2.width() {
        println!("This rectangle width is nonzero number: {}", rec2.width);
    }else {
        println!("This is width value is 0 And AREA is {}", rec2.area());
    }
}
