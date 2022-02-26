use quickcheck::quickcheck;
use quickcheck::Arbitrary;
use quickcheck_derive::Arbitrary;

#[derive(PartialEq, Clone, Debug)]
struct ForeignType(u8);

#[derive(Arbitrary, PartialEq, Clone, Debug)]
struct LocalType {
    #[arbitrary(generator = "gen_foreign_type")]
    v: ForeignType,
}

fn gen_foreign_type(g: &mut quickcheck::Gen) -> ForeignType {
    ForeignType(loop {
        let v = u8::arbitrary(g);
        if v > 20 {
            break v;
        }
    })
}

fn main() {
    fn prop_foreign_type_larger_than_20(local: LocalType) -> bool {
        local.v.0 > 20
    }
    quickcheck(prop_foreign_type_larger_than_20 as fn(_) -> _)
}
