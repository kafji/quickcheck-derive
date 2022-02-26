use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
struct Stone;

fn main() {
    fn prop_clone_is_equal_with_its_origin(person: Stone) -> bool {
        person.clone() == person
    }
    quickcheck(prop_clone_is_equal_with_its_origin as fn(Stone) -> bool)
}
