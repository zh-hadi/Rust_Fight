struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){
    let mut rec1 = Rectangle{
        width: 50,
        height: 11,
    };

    println!("{}", area( &mut rec1)); // pass by refercence
    println!("width: {}  height: {}", rec1.width, rec1.height);

}

fn area(rectangle: &mut Rectangle)-> u32{
    rectangle.width = 20;
    rectangle.height = 2;
    return rectangle.width * rectangle.height;
}





    


