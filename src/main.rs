struct Car {
    mpg: i32,
    color: String,
    top_speed: i32,
}
impl Car {
    fn set_mpg(&mut self, mpg: i32) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, topSpeed: i32) {
        self.top_speed = topSpeed;
    }
}

fn main() {
    let mut car = Car {
        mpg: 120,
        color: String::from("Red"),
        top_speed: 210,
    };
    car.set_mpg(150);
    car.set_color(String::from("Blue"));
    car.set_top_speed(220);
    println!("{:?} {} {}", car.color, car.mpg, car.top_speed);
}

// fn add_two(val: &mut i8) -> i8 {}
