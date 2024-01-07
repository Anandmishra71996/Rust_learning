trait carFunctions {
    fn set_mpg(&mut self, val: i32);
    fn set_color(&mut self, val: String);
    fn set_top_speed(&mut self, val: i32);
}
struct Car {
    mpg: i32,
    color: String,
    top_speed: i32,
}
impl carFunctions for Car {
    fn set_mpg(&mut self, val: i32) {
        self.mpg = val;
    }
    fn set_color(&mut self, val: String) {
        self.color = val;
    }
    fn set_top_speed(&mut self, val: i32) {
        self.top_speed = val
    }
}
fn main() {
    let mut newCar = Car {
        mpg: 150,
        top_speed: 220,
        color: String::from("Red"),
    };
    println!("{} {} {}", newCar.mpg, newCar.top_speed, newCar.color);
    newCar.set_mpg(70);
    newCar.set_top_speed(250);
    newCar.set_color(String::from("Green"));
    println!("{} {} {}", newCar.mpg, newCar.top_speed, newCar.color);
}
