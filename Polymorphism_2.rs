enum Option<T>
{
    Some(T),
    None
}

fn main()
{
    let a: Option<i8> = Some(5);
    let p: Option<f64> = Some(3.14159265359);
}