fn main()
{
    let n = 1;
    match n 
    {
        ref r => println!("{}", r),
    }
    let mut m = 11;
    match m
    {
        ref mut mr =>
        {
            println!("{}", mr);
            *mr = 111;
        },
    }
    println!("{}",m);
}