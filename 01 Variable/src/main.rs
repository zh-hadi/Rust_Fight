

fn main() {
    // mutable variable 
    let x = 30;
    // immutable variable
    let mut y:u32 = 40;

    y = y + 10;

    {
        println!("{} + {} = {}", x, y, add(x,y));
    }



    // Scalar data types 
    // -> integer   :: signed  i8, i16, i32, i64 , i123 isize || unsigned u8, u16, u32, u64, usize
    // -> float number :: f32, i64 
    // -> boolean :: true , false
    // -> catrecter :: char 

    //float number
    // -- declear method 1
    //let o:f32 = 2.1;
    //let oo:f64 = 23.3;
    // -- declear method 2
    let o = 2.1f32;
    let oo = 23.3f64;
    println!("{}", o);
    println!("{}", oo);

    // boolean
    let b:bool = true;
    let bb:bool = false;
    println!("{} {}", b, bb);

    // character
    let cc:char = 'H';
    println!("character value: {cc}");
}

// func :: see how to declear function

// fn function_name(parameter) return_type{statement}   :: parameter(variable_name: varible_type)
fn add(a:u32, b:u32)-> u32{
    a +b
}