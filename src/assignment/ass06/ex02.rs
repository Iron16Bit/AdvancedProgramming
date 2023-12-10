use std::ops::Deref;

#[derive(Debug, Clone)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool,
}

struct CarDealer {
    catalogue: Vec<Car>,
}

struct User {
    car: Option<Car>,
}

impl CarDealer {
    fn new(vec: Vec<Car>) -> Self {
        return CarDealer { catalogue: vec };
    }

    fn add_car(&mut self, car: Car) {
        self.catalogue.push(car);
    }

    fn print_cars(&self) {
        println!("Dealer has the following cars:");
        for c in self.catalogue.iter() {
            println!("{:?}", c);
        }
    }

    fn rent_user(&mut self, usr: &mut User, model: &str) {
        let mut found = false;
        for c in self.catalogue.iter_mut() {
            if c.model.as_str() == model {
                if !c.rent {
                    c.rent = true;
                    usr.car = Some(c.clone());
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("Car not found");
        }
    }

    fn end_rental(&mut self, usr: &mut User) {
        if usr.car.is_none() {
            println!("User has no car");
        } else {
            for c in self.catalogue.iter_mut() {
                if c.model == usr.car.as_ref().unwrap().model {
                    c.rent = false;
                    usr.car = None;
                    break;
                }
            }
        }
    }
}

impl Default for Car {
    fn default() -> Self {
        return Car {
            model: "Fiat".to_string(),
            year: 2013,
            price: 100,
            rent: false,
        };
    }
}

impl Car {
    fn new(model: &str, year: u32, price: u32) -> Self {
        return Car {
            model: model.to_string(),
            year,
            price,
            rent: false,
        };
    }
}

impl User {
    fn print_car(&self) {
        if self.car.is_none() {
            println!("User has no car");
        } else {
            println!("{:?}", self.car);
        }
    }
}

pub fn test() {
    let car1 = Car::new("Mustang", 2023, 321);
    let car2 = Car::new("Panda", 1980, 20);
    let car3 = Car::new("Mercedes", 2000, 150);
    let vec: Vec<Car> = vec![car1, car2, car3];

    let mut dealer = CarDealer::new(vec);
    dealer.add_car(Car::default());
    dealer.print_cars();

    println!("\n");
    let mut usr = User { car: None };
    usr.print_car();
    dealer.rent_user(&mut usr, "Giovanni");
    dealer.rent_user(&mut usr, "Mustang");
    usr.print_car();
    dealer.print_cars();

    println!("\n");
    dealer.end_rental(&mut usr);
    usr.print_car();
    dealer.print_cars();
}
