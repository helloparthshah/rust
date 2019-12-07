use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut flag=0;
    if n%4==0{
        flag=1;
        if n%100==0 && n%400!=0{
            flag=0;
        }
    }
    if flag==1{
        println!("{} is a leap year",n);
    }
    else{
        println!("{} is not a leap year",n);
    }
}