#[derive(Debug)]
struct My {
    field1: i32
}

impl Drop for My {
    fn drop(&mut self) {
        println!("{the_field:?} is dropping", the_field = self)    
    }
}

fn move_it(_my: My) {}

fn borrow_it(_my: &My) {}

fn main() {
    let my = My { field1: 1 };
    println!("0");
    borrow_it(&my);
    println!("1");
    move_it(my);
    println!("2")
}
