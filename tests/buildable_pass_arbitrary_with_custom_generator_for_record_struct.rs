use quickcheck::quickcheck;
use quickcheck::Arbitrary;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
struct Person {
    name: String,

    #[arbitrary(generator = "gen_age")]
    age: u8,
}

fn gen_age(g: &mut quickcheck::Gen) -> u8 {
    loop {
        let age = u8::arbitrary(g);
        if age > 16 {
            break age;
        }
    }
}

fn main() {
    fn prop_can_vote(person: Person) -> bool {
        person.age >= 17
    }
    quickcheck(prop_can_vote as fn(_) -> _)
}
