use quickcheck::quickcheck;
use quickcheck::Arbitrary;
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, PartialEq, Clone, Debug)]
enum Vehicle {
    Car(u8),
    Bike(#[arbitrary(generator = "gen_bike_capacity")] u8),
}

fn gen_bike_capacity(g: &mut quickcheck::Gen) -> u8 {
    loop {
        let cap = u8::arbitrary(g);
        if cap < 3 {
            break cap;
        }
    }
}

fn main() {
    fn prop_bike_can_only_fit_two_persons(vehicle: Vehicle) -> bool {
        if let Vehicle::Bike(cap) = vehicle {
            cap <= 2
        } else {
            true
        }
    }
    quickcheck(prop_bike_can_only_fit_two_persons as fn(Vehicle) -> bool)
}
