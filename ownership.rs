struct two_num
{
    one: i32,
    two: i32
}

fn main()
{
    let mut a = two_num{ one: 1, two: 2};
    {
        let b = &mut a;
        println!("{}",b.one);
    }
    a.one = 100;
    println!("{}",a.one);
}