use quickcheck::quickcheck;
use quickcheck::Arbitrary;
use quickcheck_derive::Arbitrary;
use serde::Serialize;

#[derive(Serialize, Arbitrary, PartialEq, Clone, Debug)]
struct Velocity {
    #[serde(rename = "foreign_type")]
    #[arbitrary(generator = "gen_v")]
    v: u8,
}

fn gen_v(g: &mut quickcheck::Gen) -> u8 {
    loop {
        let v = u8::arbitrary(g);
        if v > 20 {
            break v;
        }
    }
}

fn main() {
    fn prop_foreign_type_larger_than_20(velocity: Velocity) -> bool {
        velocity.v > 20
    }
    quickcheck(prop_foreign_type_larger_than_20 as fn(_) -> _)
}
