use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Lanternfish {
    timer: u8,
}

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish { timer: 8 }
    }

    fn day_tick(&mut self) -> Option<Lanternfish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(Lanternfish::new());
        }
        self.timer -= 1;
        None
    }
}

impl fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.timer)
    }
}

impl FromStr for Lanternfish {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timer = ok_or_return_s!(s.parse::<u8>());
        Ok(Lanternfish { timer })
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let mut lanternfishes: Vec<Lanternfish> = input
        .split(",")
        .map(Lanternfish::from_str)
        .map(Result::unwrap)
        .collect();
    // let recreated_input = lanternfishes.iter()
    //     .map(|f| format!("{}", f))
    //     .collect::<Vec<_>>()
    //     .join(",");
    // assert_eq!(recreated_input, input);

    for _day in 1..=80 {
        let mut children: Vec<Lanternfish> = vec![];
        for lanternfish in &mut lanternfishes {
            if let Some(child) = lanternfish.day_tick() {
                children.push(child);
            }
        }
        lanternfishes.append(&mut children);
    }

    return lanternfishes.len() as i32;
}

#[derive(Copy, Clone, Debug)]
struct LanternfishReproductionModel {
    fish_population: u64,
    reproduction_on_future_days: [u64; 9],
}

impl LanternfishReproductionModel {
    fn new() -> LanternfishReproductionModel {
        LanternfishReproductionModel {
            fish_population: 0,
            reproduction_on_future_days: [0; 9],
        }
    }

    fn add_fish(&mut self, days_until_reproduction: usize) -> Result<(), String> {
        self.fish_population += 1;
        if days_until_reproduction > 8 {
            return Err("fish had too many days until reproduction".to_string());
        }
        self.reproduction_on_future_days[days_until_reproduction] += 1;
        return Ok(());
    }

    fn day_tick(&mut self) {
        let new_children = self.reproduction_on_future_days[0];
        for i in 1..=8 {
            self.reproduction_on_future_days[i - 1] = self.reproduction_on_future_days[i];
        }
        self.fish_population += new_children;
        self.reproduction_on_future_days[6] += new_children;
        self.reproduction_on_future_days[8] = new_children;
    }
}

impl FromStr for LanternfishReproductionModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut model = LanternfishReproductionModel::new();
        for t in s.split(",") {
            let n = ok_or_return_s!(t.parse::<usize>());
            ok_or_return!(model.add_fish(n));
        }
        Ok(model)
    }
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let mut model = LanternfishReproductionModel::from_str(input).unwrap();

    for _day in 1..=256 {
        model.day_tick();
        //println!("After day {}: {}", day, model.fish_population);
    }

    return model.fish_population;
}
