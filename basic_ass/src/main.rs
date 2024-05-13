fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;

    println!{"{}",ans};

    let mut vec = vec![2,4,6,8,10];
    vec.pop();
    vec.push(12);
    vec.push(14);
    println!{"{:?}",vec};

    let str1 = String::from("Hello ");
    let ans = concat_string(str1);

    println!{"{}",ans};
    control_flow(27);
}

fn concat_string(val: String) -> String{
    val + "world"
}

fn control_flow(val:i32){
    if val == 1 {
        println!("The value is one");
    }else if val>50 {
        println!("value greater than 50");
    }else if val < 25 {
        println!("Value is less than 25");
    }else {
        println!("value between 25 and 50");
    }
}
