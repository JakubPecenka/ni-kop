
#[derive(Debug, Default)]
pub struct Problem {
    name: String,
    weights: Vec<i32>,
    clauses: Vec<Vec<i32>>,
    max_w_sum: i32,
}

impl Problem {
    pub fn new(name: &str, weights: Vec<i32>, clauses: Vec<Vec<i32>>) -> Self {

        let sum = weights.iter().sum::<i32>();
        Self {
            name: name.to_string(),
            weights,
            clauses,
            max_w_sum: sum,
        }
    }
}

impl TryFrom<&str> for Problem {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input = input.split("\n").collect::<Vec<&str>>();

        let mut weigths = input[9].split(" ")
        .skip(1)
        .map(|w| w.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
        weigths.pop();

        let sum = weigths.iter().sum::<i32>();

        let name = input[7].split(" ").nth(3).unwrap().to_string();

        let clauses = input[11..].iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut curr_clauses = line.trim().split(" ")
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            curr_clauses.pop();
            curr_clauses
        }).collect::<Vec<_>>();

        Ok(Self {
            name,
            weights: weigths,
            clauses,
            max_w_sum: sum,
        })
    }
}