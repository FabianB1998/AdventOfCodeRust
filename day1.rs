use std::collections::HashSet;

pub fn main() {
    //Lines ist ein iterator ueber alle zeilen
    let lsg_1 = data.lines()
    //addiert alle elemente des iterators (fold ermoeglicht modifizierung eines wertes ueber alle zeilen)
    .fold(0, |acc, line| acc + line.parse::<i32>().unwrap());
    
    println!("1 = {}", lsg_1);
    
    let mut set = HashSet::new();
    
    let mut ctr = 0;
    
    //Cycle macht einen endlosen iterator, der sich zuruecksetzt
    data.lines().cycle()
    //Bricht ab, sobald die bedingung erfuellt ist
    .any(|line|{ ctr +=  line.parse::<i32>().unwrap();
                                    //wenn ein item bereits im set ist, returned insert false
                                    !set.insert(ctr)});
    println!("2 = {}", ctr);
}

const data: &str = include!("data.include");