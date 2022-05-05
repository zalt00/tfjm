use std::collections::{VecDeque, HashSet, HashMap};

const MAX_SIZE: usize = 15;

pub struct City<const SIZE: usize> {
    matrix: [[bool; SIZE]; SIZE],
    population: usize,
    friends: Vec<HashSet<usize>>
}

impl<const SIZE: usize> City<SIZE> {
    pub fn new(population: usize) -> City<SIZE> {
        City {matrix: [[false; SIZE]; SIZE], population, friends: vec![]}
    }

    pub fn precalculate_friends(&mut self) {
        for i in 0..self.population {
            self.friends.push(self.get_friends(i))
        }
    }

    pub fn get_precalculated_friends(&self, i: usize) -> Option<&HashSet<usize>> {
        self.friends.get(i)
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.matrix[a][b] = true;
        self.matrix[b][a] = true;
    }

    pub fn add_cycle(&mut self, start: usize, stop: usize) -> &mut City<SIZE> {
        self.add_edge(start, stop - 1);

        for i in (start + 1)..stop {
            self.add_edge(i, i - 1)
        } self
    }

    pub fn get_population(&self) -> usize {self.population}

    pub fn get_friends(&self, i: usize) -> HashSet<usize> {
        let mut output = HashSet::with_capacity(SIZE);
        for (j, is_friend) in self.matrix[i].iter().enumerate() {
            if *is_friend && j != i{
                output.insert(j);
            }
        }; output
    }

    pub fn are_friends(&self, i: usize, j: usize) -> bool {
        self.matrix[i][j]
    }

}


pub struct Links<const SIZE: usize> {
    city_a: City<SIZE>,
    city_b: City<SIZE>,
    matrix: [[Option<usize>; SIZE]; 2],
    current_hash: u128
}

impl<const SIZE: usize> Links<SIZE> {
    pub fn new(city_a: City<SIZE>, city_b: City<SIZE>) -> Links<SIZE> {
        Links { city_a, city_b, matrix: [[None; SIZE]; 2], current_hash: 0}
    }

    pub fn get_neighbor(&self, i: usize, city_b: bool) -> Option<usize> {
        self.matrix[city_b as usize][i]
    }

    pub fn checked_add_link(&mut self, pa: usize, pb: usize) -> bool {
    
    
        self.current_hash += ((pb + 1) << (pa * 4)) as u128;
    
    
        self.matrix[0][pa] = Some(pb);
        self.matrix[1][pb] = Some(pa);

        let friends_of_a = self.city_a.get_precalculated_friends(pa).unwrap();
        let friends_of_b = self.city_b.get_precalculated_friends(pb).unwrap();

        for friend in friends_of_a.iter() {
            match self.get_neighbor(*friend, false) {
                Some(linked) => if !(friends_of_b.contains(&linked)) {return false},
                _ => ()
            }
        } true
    }

    pub fn remove_link(&mut self, pa: usize, pb: usize) {
    
        self.current_hash -= ((pb + 1) << (pa * 4)) as u128;

        self.matrix[0][pa] = None;
        self.matrix[1][pb] = None;
    }
    
    pub fn get_current_hash(&self) -> u128 {
        self.current_hash
    }

}



pub fn explore(links: &mut Links<MAX_SIZE>, not_linked_a: &mut VecDeque<usize>, not_linked_b: &mut VecDeque<usize>, number_of_calls: &mut usize, memoization_table: &mut HashMap<u128, u8>, stop_at: usize) -> usize {
    
    if memoization_table.contains_key(&links.get_current_hash()) {
        return *(memoization_table.get(&links.get_current_hash()).unwrap()) as usize
    };
    
    let mut current_max: usize;
    let mut current_min: usize = MAX_SIZE + 42;
    let mut person_a: usize;
    let mut person_b: usize;

    *number_of_calls += 1;

    for _i in 0..not_linked_a.len() {
        person_a = not_linked_a.pop_back().unwrap();
        current_max = 0;
        for _j in 0..not_linked_b.len() {
            person_b = not_linked_b.pop_back().unwrap();

            if links.checked_add_link(person_a, person_b) {
                current_max = usize::max(explore(links, not_linked_a, not_linked_b, number_of_calls, memoization_table, current_max) + 1, current_max);
                if (current_max >= current_min){
                    not_linked_b.push_front(person_b);
                    links.remove_link(person_a, person_b);
                    break
                }
            }

            not_linked_b.push_front(person_b);

            links.remove_link(person_a, person_b)

        }
        not_linked_a.push_front(person_a);

        current_min = usize::min(current_min, current_max);
        if (current_min <= stop_at){
            break
        }

    };
    
    let output = if current_min > MAX_SIZE {0} else {current_min};
    if memoization_table.len() < 200000000 {
        memoization_table.insert(links.get_current_hash(), output.clone() as u8);
    }
    output
}


fn main() {

    let mut city_a = City::new(10);
    city_a.add_cycle(0, 5).add_cycle(5, 10);

    city_a.precalculate_friends();

    let mut city_b = City::new(10);
    city_b.add_cycle(0, 10);

    city_b.precalculate_friends();

    let mut not_linked_a = VecDeque::with_capacity(MAX_SIZE);
    for i in 0..city_a.get_population() {
        not_linked_a.push_back(i)
    }

    let mut not_linked_b = VecDeque::with_capacity(MAX_SIZE);
    for i in 0..city_b.get_population() {
        not_linked_b.push_back(i)
    }    
    let mut links: Links<MAX_SIZE> = Links::new(
        city_a,
        city_b,
    );


    let mut number_of_calls = 0_usize;
    let mut table: HashMap<u128, u8> = HashMap::new();
    
    let max_capa = explore(&mut links, &mut not_linked_a, &mut not_linked_b, &mut number_of_calls, &mut table, 0);
    println!("Max capacity: {}, num: {}", max_capa, number_of_calls)




}
