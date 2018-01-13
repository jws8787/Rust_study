fn again<F: Fn(i32) -> i32>(f: F, s: i32) -> i32 
{
    f(f(s))
}