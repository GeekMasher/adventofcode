use crate::Arguments;
use anyhow::Result;

type Loc = u64;

fn number_split(data: &str) -> Vec<Loc> {
    data.split(' ')
        .filter_map(|r| match r.parse::<Loc>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect()
}

#[derive(Debug, Default)]
struct Range {
    destination: Loc,
    source: Loc,
    length: Loc,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let result = number_split(value);
        Range {
            destination: result[0],
            source: result[1],
            length: result[2],
        }
    }
}

impl Range {
    pub fn lookup(&self, curr: Loc) -> Option<Loc> {
        if curr >= self.source && curr < self.source + self.length {
            return Some((curr - self.source) + self.destination);
        }
        None
    }
}

#[derive(Debug, Default)]
struct Almanac {
    pub seeds: Vec<Loc>,
    pub locations: Vec<(String, Vec<Range>)>,
}

impl Almanac {
    pub fn build(&mut self, input: &str) {
        let mut current_name = String::new();
        let mut current_locs: Vec<Range> = Vec::new();

        for line in input.lines() {
            if line.starts_with("seeds") {
                self.seeds = number_split(&line.replace("seeds: ", ""));
            } else if line.is_empty() && !current_name.is_empty() {
                self.locations.push((current_name, current_locs));

                current_name = String::new();
                current_locs = Vec::new();
            } else if line.ends_with("map:") {
                current_name = line.replace(" map:", "");
            } else if !line.is_empty() {
                current_locs.push(Range::from(line));
            }
        }

        if !current_name.is_empty() {
            self.locations.push((current_name, current_locs));
        }
    }

    pub fn lookup(&self, seed: Loc) -> Vec<Loc> {
        let mut positions: Vec<Loc> = vec![seed];
        let mut last = seed;

        for (_name, ranges) in &self.locations {
            // println!("{name}");
            let mut found = false;

            for range in ranges {
                if let Some(r) = range.lookup(last) {
                    found = true;
                    last = r;
                    positions.push(r);
                    break;
                }
            }

            if !found {
                positions.push(last);
            }
        }
        positions
    }
}

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;

    let mut almanac = Almanac::default();
    almanac.build(&input);

    println!("Seeds :: {:?}", almanac.seeds);

    let mut lowest: Loc = 0;

    for seed in &almanac.seeds {
        // let seed = almanac.seeds[0];
        println!("Finding location for :: {seed}");
        let locs = almanac.lookup(*seed);

        if let Some(last) = locs.last() {
            if lowest == 0 {
                lowest = *last;
            } else if last < &lowest {
                lowest = *last;
            }
        }

        println!("{:?}", locs);
    }

    println!("Lowest :: {lowest}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day5::Range;

    #[test]
    fn range_lookup() {
        let range = Range {
            destination: 55,
            source: 5,
            length: 2,
        };
        assert_eq!(range.lookup(2), None);
        assert_eq!(range.lookup(5), Some(55));
        assert_eq!(range.lookup(6), Some(56));
        assert_eq!(range.lookup(7), None);
    }
}
