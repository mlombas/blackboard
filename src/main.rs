use blackboard::{BlackBoard, Subscriptor};

struct Kitchen { 
    oven: Vec<String>,
}

impl Kitchen {
    fn roast(&mut self, food: &mut &str) {
        let mut s = String::from("Roasted");
        s.push_str(" ");
        s.push_str(food);

        self.oven.push(s);
    }

    fn bake(&mut self, food: &mut &str) {
        let mut s = String::from("Baked");
        s.push_str(" ");
        s.push_str(food);

        self.oven.push(s);
    }
}

impl Subscriptor<&str> for Kitchen {
    fn notify(&mut self, food: &mut &str) {
        println!("jaja");
        if ["Potato", "Cake", "Cookies"].contains(food) {
            self.bake(food) 
        } else {
            self.roast(food) 
        }
    }
}

fn main() {
    let mut restaurant_blackboard = BlackBoard::new();

    restaurant_blackboard.subscribe("Tables", |s: &mut _| { println!("{}", s); } );
    
    //Simulate restaurant
    restaurant_blackboard.post("Kitchen", "Potato");
    restaurant_blackboard.post("Entrance", "New Customer");
    restaurant_blackboard.post("Kitchen", "Chicken");
    restaurant_blackboard.post("Tables", "Table 25 free");
    restaurant_blackboard.post("Parking", "Parking is full");
    restaurant_blackboard.post("Kitchen", "Cookies");
    restaurant_blackboard.post("Tables", "Table 2 needs waiter");
    restaurant_blackboard.post("Kitchen", "Rice");
}
