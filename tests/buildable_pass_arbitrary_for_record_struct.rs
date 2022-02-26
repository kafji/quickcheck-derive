use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    fn prop_clone_is_equal_with_its_origin(person: Person) -> bool {
        person.clone() == person
    }
    quickcheck(prop_clone_is_equal_with_its_origin as fn(Person) -> bool)
}
