struct Player
{
    nname: &'static str,
    helath: i32,
    level: u8
}
fn main()
{
    let mut p = Player{ nname: "jws", helath: 100, level: 1};
    let Player{helath: ht, nname: nn, ..} = p;
    println!("{} {}",nn, ht);
}