use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
struct Stone;

fn main() {
    fn clone_equal_with_origin(person: Stone) -> bool {
        person.clone() == person
    }
    quickcheck(clone_equal_with_origin as fn(Stone) -> bool)
}
