use std::hash::Hash;
use std::collections::HashMap;

///Represents a blackboard where processes can post and get items in sections, and also subscribe
///for changes in a specific section.
pub struct BlackBoard<'a, Section: Hash + Eq, Type> {
    map: HashMap<
        Section,
        (Vec<Type>, Vec<Box<dyn FnMut(&Type) + 'a>>)>
}

impl<'a, Section: Hash + Eq, Type> BlackBoard<'a, Section, Type> {
    ///Generates a new, empty blackboard
    pub fn new() -> Self {
        BlackBoard { map: HashMap::new() }
    }

    ///Gets the section, if it exists.
    pub fn get(&self, section: &Section) -> Option<&Vec<Type>> {
        Some(
            &self.map
                .get(section)? //Get the section of return none if not exists
                .0 //First element, the type
            )
    }

    ///Returns a list of all the sections present right now in the BlackBoard
    pub fn get_sections(&self) -> Vec<&Section> {
        self.map.keys().collect()
    }

    ///Posts data on the desired section. Note that data is consumed
    pub fn post(&mut self, section: Section, what: Type) {
        let (things, events) = self.get_raw_section(section);

        for event in events {
            event(&what);
        }

        things.push(what);
    }

    ///Suscribes a function to the specified section. When a post to that section occurs, the
    ///function passed in is called with the newly posted value as argument
    pub fn subscribe(&mut self, section: Section, on_event: impl FnMut(&Type) + 'a) {
        self.get_raw_section(section)
            .1
            .push(
                Box::new(on_event)
            );
    }

    fn get_raw_section(&mut self, section: Section) -> 
        &mut (Vec<Type>, Vec<Box<dyn FnMut(&Type) + 'a>>) 
    {
        self.map.entry(section)
            .or_insert((
                Vec::new(),
                Vec::new(),
                ))
    }
}

//Its important to drop as soon as we can so closures dont mess up everything
impl<'a, Section: Hash + Eq, Type> Drop for BlackBoard<'a, Section, Type> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_and_get() {
        let mut bb = BlackBoard::new();

        bb.post("Street", 25);
        bb.post("Home", 15);
        bb.post("Street", 82);

        assert!(
            {
                let mut all_are_in = true;
                let expected = vec!["Street", "Home"];

                for section in 
                    bb.get_sections()
                {
                    if !expected.contains(&section) { 
                        all_are_in = false;
                        break;
                    }
                }

                all_are_in
            }
        );

        assert_eq!(
            &vec![25, 82],
            bb.get(&"Street").unwrap()
        );
        assert_eq!(
            &vec![15],
            bb.get(&"Home").unwrap()
        );
    }

    #[test]
    fn suscribe_test() {
        let mut bb = BlackBoard::<&str, &str>::new();
        
        static mut changed: bool = false;
        fn change(_: &&str) {
            unsafe { changed = true }
        }

        bb.subscribe("Park", change);

        bb.post("Park", "");

        unsafe { assert!(changed); }
    }
}
