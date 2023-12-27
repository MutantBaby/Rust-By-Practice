trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn _fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    let duck: Duck = Duck;
    duck.swim();

    let bird: Box<dyn Bird> = hatch_a_bird(1);
    assert_eq!(bird.quack(), "duck duck");

    let bird: Box<dyn Bird> = hatch_a_bird(2);
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}
fn hatch_a_bird(num: i32) -> Box<dyn Bird + 'static> {
    match num {
        1 => Box::new(Duck),
        2 => Box::new(Swan),
        _ => panic!("Error"),
    }
}
