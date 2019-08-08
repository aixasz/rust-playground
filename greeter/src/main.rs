fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 6;

    println!("X is :{} {}", x, y);

    let t =  true;

    println!("true : {}", t);

    let cat = '🐈';

    println!("{}", cat);
}


#[cfg(test)]
mod tests{
    #[test]
    fn it_work(){
        assert_eq!(2 + 2, 4);
    }
}