fn main()
{
    let mut a = 1..10;
    loop 
    {
        match a.next()
        {
            Some(x) =>
            {
                print!("{}",x);
            }
            None =>
            {
                break
            }
        }
    }
}