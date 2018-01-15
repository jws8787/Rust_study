struct Animal
{
    health: u32,
    damage: u32
}

fn main()
{
    let mut cat = Animal{ health: 50, damage: 1};
    impl Animal
    {
        fn new(mut h: u32, d: u32) -> Animal
        {
            if h > 100 { h = 100; }
            Animal {health: h, damage: d};
        }
    }
    let mut dog = Animal::new(20000, 15);
}