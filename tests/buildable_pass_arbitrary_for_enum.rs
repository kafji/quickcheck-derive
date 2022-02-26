use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
enum Vehicle {
    Airplane { velocity: u16, altitude: i32 },
    Scooter(u16, i32),
    Rock,
}

fn main() {
    fn prop_clone_is_equal_with_its_origin(person: Vehicle) -> bool {
        person.clone() == person
    }
    quickcheck(prop_clone_is_equal_with_its_origin as fn(Vehicle) -> bool)
}
