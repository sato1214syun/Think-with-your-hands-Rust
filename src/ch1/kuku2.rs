fn main(){
    for y in 1..=9{
        let s = (1..=9)
                .map(|x| format!("{:3}", x * y))
                .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}