fn main(){
    for y in 1..=9{
        for x in 1..=9 {
            print!("{:3}, ", x * y);
        }
        println!();
    }
}