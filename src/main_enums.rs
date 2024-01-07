enum shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}
impl shape {
    fn corners(&self) -> i8 {
        match self {
            shape::Triangle => 3,
            shape::Square => 4,
            shape::Pentagon => 5,
            shape::Octagon => 8,
            _ => 0,
        }
    }
}
fn main() {
    let sq = shape::Square;
    let tr = shape::Triangle;
    let val = sq.corners();
    let val1 = tr.corners();
    println!("{}", val);
    println!("{}", val1);
}
