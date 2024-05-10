fn main() {
    // let x: i8 = 10;
    // println!("{}",x);

    // let y: u8 = 10;

    // let tup: (i32, &str, bool) = (500,"naboth", true);
    // println!("{}", tup.2);

    // let (x,y,z) = tup;
    // println!("{}",x);
    // println!("{}",y);
    // println!("{}",z);

    // let array = [1,2,3];
    // println!("{}",array[1]);

    // let mut array2 = [4,5,6];
    // println!("{}",array2[1]);
    // array2[0] = 44;
    // println!("{}",array2[0]);

    // let mut nums = vec![1,2,3];
    // nums.push(4);
    // println!("{:?}",nums);

    let name = "Naboth".to_string();
    println!("{}",name);
    let course = String::from("Rust");
    println!("{}",course);
    let new_name = name.replace("Naboth", "DonTech Solutions");
    println!("{}",new_name);
}
