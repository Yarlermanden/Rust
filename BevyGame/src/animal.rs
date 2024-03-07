//traits
//Think more of trait as some behavior to describe than something to be
trait Living {
    fn duplicate(&self) -> Self;
    fn replicate(&self, other: Self) -> Self;
    fn die(&mut self); //set to dead
    fn eat(&mut self, amount: i32); //decrease hunger and possibly increase hp
}


//Structs ----------------------------------------------------------------

//Entity related to functionality 
struct LivingEntity {
    alive: bool,
    hp: i32,
    hunger: i32,
}

struct VisibleEntity {
    x_position: i32,
    y_position: i32,
    //skins/texture
}

//Sheep struct
struct Sheep { 
    alive: bool,
    hp: i32,
    hunger: i32,
}

impl Sheep {
    //grass
}

//Like a default constructor
impl Default for Sheep {
    fn default() -> Self { Sheep { alive:true, hp:100, hunger:0 } }
}

//Wolf struct
struct Wolf { 
    alive: bool,
    hp: i32,
    hunger: i32,
    attack_power: i32,
}

impl Wolf {
    //attack - can call eat, if it successfully kill a sheep
}

impl Default for Wolf {
    fn default() -> Self { Wolf { alive:true, hp:50, hunger:0, attack_power: 10 } }
}

//Implementation of traits
impl Living for Sheep {
    fn duplicate(&self) -> Sheep {
        return Sheep {..Default::default() };
    }

    fn replicate(&self, other: Sheep) -> Sheep{
        let hp = (self.hp + other.hp) / 2;
        return Sheep {hp: hp, ..Default::default()};
    }

    fn die(&mut self) {
        //do specific sheep cleanup here... invoke death animation...
        self.alive = false;
    }

    fn eat(&mut self, amount: i32) {
        self.hunger-= amount;
    }
}

impl Living for Wolf {
    fn duplicate(&self) -> Wolf {
        return Wolf {..Default::default() };
    }

    fn replicate(&self, other: Wolf) -> Wolf {
        let hp = (self.hp + other.hp) / 2;
        let ap = other.attack_power; //change to something between the two
        return Wolf {hp: hp, attack_power:ap, ..Default::default()};
    }

    fn die(&mut self) {
        //do specific sheep cleanup here... invoke death animation...
        self.alive = false;
    }

    fn eat(&mut self, amount: i32) {
        self.hunger-= amount;
    }
}

//todo can refactor some of this into a subtype, which has many of the traits, which both then use