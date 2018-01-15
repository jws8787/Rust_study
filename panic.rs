fn main()
{
    let a = 10;
    let b = 0;
    if(b == 0)
    {
        panic!("정수를 0으로 나눌 수 없습니다.");
    }
    println!("{}", div(a, b));
}

fn div(x: i32, y: i32) -> f32
{
    (x / y) as f32
}