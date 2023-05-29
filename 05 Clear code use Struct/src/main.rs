fn main(){
    let rec1:(u32, u32) = (32, 30);

    println!("{}", area(rec1));

}

fn area(rectangle: (u32, u32))-> u32{
    rectangle.0 * rectangle.1
}





    


