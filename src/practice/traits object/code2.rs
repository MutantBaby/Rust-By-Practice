trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn _fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];
    // let birds2: [&dyn Bird; 2] = [&Duck, &Swan]; // 2nd approach

    for bird in birds {
        bird.quack();
    }
}
