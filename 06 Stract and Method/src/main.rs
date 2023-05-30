#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Reactangle{
        width: 30,
        height: 40,
    };

    dbg!(rec1);
}
