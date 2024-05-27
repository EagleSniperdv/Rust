fn main() {
    // ownership
    // let var = 1;
    // let mut s = "hello".to_string();
    // s.push_str(",world");
    // println!("{}",var);

    // move

    // let x = vec!["tyler".to_string()];
    // let y = x;
    // // let z = y;
    // println!("{:?}",y);

    // clone

    // let x = vec!["tyler".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    
    // println!("{:?}",x);
    // println!("{:?}",y);
    // println!("{:?}",z);

    // copy

    // let x =1;
    // let y = x;

    // println!("x = {}, y = {}",x,y);
    
    let s = String::from("takes");
    // println!("{}",s);
    takes_ownership(s);

}

fn takes_ownership(s: String) {
    let strin = s;
    println!("{}",strin);
}

