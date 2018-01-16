fn main()
{
    let n = 10;
    let n2 = n + 5;
    let result = sum(n, n2);
    println!("{}",result);
    {
        let time = 0;
    }
}

fn sum(x: i32, y: i32) -> i32
{
    x + y
}