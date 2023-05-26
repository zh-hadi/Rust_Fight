fn main() {
    // if else condition
    let a = 2;
    if a > 10{
        println!("{} is biger than 10", a);
    }else if a > 5 {
        println!("{} is biger than 5 ", a);
    }else{
        println!("{} is less than 5", a);
    }

    // if else use let 
    let num = if 20 > 150 {20} else {150};
    println!("number is {} ", num);


    
}
