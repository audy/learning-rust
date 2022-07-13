#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn double(&mut self) {
        self.width = self.width * 2;
        self.height = self.height * 2;
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Self) -> bool {
        (self.width > other_rect.width) && (self.height > other_rect.height)
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    println!(
        "{:?} can hold {:?}: {:?}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );

    rect1.double();

    println!(
        "{:?} can hold {:?}: {:?}",
        rect1,
        rect2,
        rect1.can_hold(&rect2)
    );

    let square = Rectangle::square(10);

    println!("square={:#?}", square);
}
