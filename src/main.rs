use blackboard::BlackBoard;

fn main() {
    let mut milk_acquired = 0;
    let mut found_betsie = false;

    {
        let mut barn_blackboard = BlackBoard::new();

        barn_blackboard.subscribe("Cows", |_| { milk_acquired += 1 });

        barn_blackboard.subscribe("Chickens",
            |c| { 
                if *c == "Betsie" { found_betsie = true; }
            }
        );

        barn_blackboard.post("Cows", "Anna");
        barn_blackboard.post("Cows", "Clara");
        barn_blackboard.post("Chickens", "Gregory");
        barn_blackboard.post("Sheep", "Daisy");
        barn_blackboard.post("Sheep", "Rosie");
        barn_blackboard.post("Cows", "Sugar");
        barn_blackboard.post("Chickens", "Betsie");
        barn_blackboard.post("Cows", "Anna");
    }

    assert_eq!(4, milk_acquired);
    assert!(found_betsie);
}
