use rand::random;

#[derive(Debug, Copy, Clone)]
struct PublicStreetLight {
    id: u32,
    on: bool,
    burn_out: bool,
}

struct PublicIllumination {
    lights: Vec<PublicStreetLight>,
}

impl Default for PublicStreetLight {
    fn default() -> Self {

        return PublicStreetLight{id: random::<u32>(), on: false, burn_out: false};
    }
}

impl PublicStreetLight {
    fn new(id: u32) -> Self {
        return PublicStreetLight{id, on: false, burn_out: false};
    }
}

impl Default for PublicIllumination {
    fn default() -> Self {
        return PublicIllumination{lights: Vec::new()};
    }
}

impl PublicIllumination {
    fn new(vec: Vec<PublicStreetLight>) -> Self {
        return PublicIllumination{lights: vec};
    }
}

impl Iterator for PublicIllumination {
    type Item = PublicStreetLight;

    fn next(&mut self) -> Option<Self::Item> {
        return self.lights.iter() //Iter through the tasks
            .position(|light| light.burn_out) //Puts the Task we're iterating on in task and checks if the condition is verified. If so, returns its index
            .map(|light| self.lights.remove(light)); //Takes that index and removes that value
    }
}

pub fn test() {
    let lights = vec![PublicStreetLight::default(), PublicStreetLight::default(), PublicStreetLight::default(), PublicStreetLight::default()];
    let lights1 = PublicIllumination::new(lights);
    for l in lights1 {
        println!("{:?}", l);
    }

    println!("Step 2:");

    let mut lights2 = vec![PublicStreetLight{id: 1234, on: false, burn_out: true}];
    lights2.push(PublicStreetLight{id: 5678, on: false, burn_out: false});
    lights2.push(PublicStreetLight{id: 9101, on: false, burn_out: false});
    lights2.push(PublicStreetLight{id: 1213, on: false, burn_out: true});
    let l2 = PublicIllumination::new(lights2);
    for l in l2 {
        println!("{:?}", l);
    }
}