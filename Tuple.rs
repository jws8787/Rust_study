fn animal(name: &str, leg: u32) -> (&str, u32)
{
    if leg == 4
    {
        return (name, leg);
    } else 
    {
        return ("dog", 4);
    }
}
fn main()
{
    let cat = ("Cat", 4u32);
    let (n, l) = animal(cat.0, cat.1);
    println!("{} is leg {}",n, l);
}