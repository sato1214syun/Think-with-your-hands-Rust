fn main(){
    let mut a = 1;
    let mut b = 1;
    for _i in 0..=30 {
        println!("{}", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
}