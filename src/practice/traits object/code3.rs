// FILL in the blanks.
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn main() {
    let x: Box<dyn Draw> = Box::new(1.1f64);
    let y: &dyn Draw = &8u8;

    draw_with_box(x);
    draw_with_ref(y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(y: &dyn Draw) {
    y.draw();
}
