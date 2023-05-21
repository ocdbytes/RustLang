struct Temperature {
    degrees_f: f64,
}
impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }
    fn show_temp(&self) {
        println!("{:?} deg F", self.degrees_f);
    }
}

enum BoxColor {
    Black,
    Red,
}
struct Box {
    dimensions: (i32, i32),
    weight: f32,
    color: BoxColor,
}
impl Box {
    fn create_new_box(dimensions: (i32, i32), weight: f32, color: BoxColor) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn get_characteristics(&self) {
        println!(
            "Dimensions : {:?} x {:?}",
            self.dimensions.0, self.dimensions.1
        );
        println!("Weight : {:?}", self.weight);
        match self.color {
            BoxColor::Black => println!("Color : Black"),
            BoxColor::Red => println!("Color : Red"),
        };
    }
}

pub(crate) fn main() {
    let hot = Temperature { degrees_f: 99.95 };
    hot.show_temp();

    let freeze = Temperature::freezing();
    freeze.show_temp();

    // BOX

    let box1 = Box::create_new_box((4, 5), 45.898, BoxColor::Black);
    let box2 = Box::create_new_box((8, 9), 20.898, BoxColor::Red);
    println!("Box 1");
    box1.get_characteristics();
    println!("Box 2");
    box2.get_characteristics();
}
