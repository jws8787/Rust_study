struct value<T>
{
    one: T,
    two: T
}

fn main()
{
    let value1: value<u32> = value {one: 100, two: 200};
    let value2: value<&str> = value {one: "one", two: "two"};
}