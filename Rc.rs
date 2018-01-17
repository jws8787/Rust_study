use std::rc::Rc;

#[derive(Debug)]
struct Animal
{
    name: String,
    leg: i32
}

#[derive(Debug)]
struct At
{
    speed: i32,
    ow: Rc<Animal>
}

fn main()
{
    let dog = Animal { name: "asd".to_string(), leg: 4};
    let dog_master = Rc::new(dog);
    for i in 1i32..dog_master.leg
    {
        let x = At {speed: i*2, ow: dog_master.clone() };
        println!("{:?}", x);
    }
}