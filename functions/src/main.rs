fn main() {
    println!("{}",gcd(50, 30));
    println!("{}",multi_returns(true));
}

fn gcd(mut a: u64,mut b: u64)-> u64
{
    while a != 0{
        if a < b {
            let c=a;
            a=b;
            b=c;
        }
        a=a%b;
        
    }
    b
}

fn multi_returns(flag: bool)->bool
{
    if flag == true {
        true
    } else {
        false
    }
}