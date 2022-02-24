use quickcheck::quickcheck;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
enum Vehicle {
    Airplane { velocity: u16, altitude: i32 },
    Scooter(u16, i32),
    Rock,
}

fn main() {
    fn clone_equal_with_origin(person: Vehicle) -> bool {
        person.clone() == person
    }
    quickcheck(clone_equal_with_origin as fn(Vehicle) -> bool)
}
