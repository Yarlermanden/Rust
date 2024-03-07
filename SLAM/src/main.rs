fn main() {
    println!("Hello, world!");

    //unknowns - nodes (x1, x2, x3) - things we can measure at different times - each frame at each time is a node - could also be other measurements.
    //relative things we know at specific time snaps
    //also our positions...

    //landmarks - (l1, l2, l3) - points in the map - the actual things we observe - the real world - like a desk or something

    //we aim at mapping the landmarks across the nodes
    let mut s = Sheep { hp: 10};
    println!("{}", s.hp);
    s.take_damage(2);
    println!("{}", s.hp);
}


struct LandMark {
    position: Point,
}

struct Node {
    pixel: Pixel,
    
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn dist(self, other: Point) -> f64 {
        (2f64.powf(self.x - other.x) + 2f64.powf(self.y - other.y) + 2f64.powf(self.z - other.z)).sqrt()
    }
    fn dist_origin(self) -> f64 {
        self.dist(Point{x: 0.0, y: 0.0, z:0.0})
    }
}

struct Pixel {
    r: i8,
    g: i8,
    b: i8
}

//TEST

struct Sheep {
    hp: i32
}

trait Alive {
    fn get_hp(&self) -> i32;
    fn set_hp(&mut self, hp: i32);
}

impl Alive for Sheep {
    fn get_hp(&self) -> i32 { self.hp }
    fn set_hp(&mut self, hp: i32) { self.hp = hp}
}

trait Damageable : Alive {
    fn take_damage(&mut self, damage: i32) {
        println!("take damage");
        self.set_hp(self.get_hp() - damage)
    }
}

impl Damageable for Sheep {}