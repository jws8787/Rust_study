fn main()
{
    let a = [2,500,400,123];
    let slc = &a[0..3];
    for i in slc
    {
        println!("{}",i);
    }
}