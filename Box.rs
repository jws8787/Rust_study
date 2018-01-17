struct two_num
{
    one: i32,
    two: i32
}

fn main()
{
    let mut a1 = Box::new(two_num{ one: 2, two:3});
    println!("{}",a1.one);
    let a2 = &mut a1;
    let num = Box::new(27);
}