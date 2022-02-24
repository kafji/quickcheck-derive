use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
struct Person(String, u8);

fn main() {
    fn clone_equal_with_origin(person: Person) -> bool {
        person.clone() == person
    }
    quickcheck(clone_equal_with_origin as fn(Person) -> bool)
}
