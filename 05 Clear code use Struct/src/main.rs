struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let rec1 = Rectangle{
        width: 50,
        height: 11,
    };

    println!("{}", area( &rec1)); // pass by refercence
    println!("width: {}  height: {}", rec1.width, rec1.height);

}

fn area(rectangle: &Rectangle)-> u32{
    rectangle.width * rectangle.height
}





    


