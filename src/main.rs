use std::fs::read_to_string;

mod simulated;
mod problem;
mod solution;

fn main() {
    let content = read_to_string("input/wuf20-71/wuf20-71-M/wuf20-0200.mwcnf").unwrap();
    println!("{}", content);

    let problem = problem::Problem::try_from(content.as_str()).unwrap();
    println!("{:?}", problem);
    
    
    let content = read_to_string("input/wuf20-71/wuf20-71-M-opt.dat").unwrap();
    let content = content.lines().collect::<Vec<_>>();
    // test jesli muzu vytvorit Solution z kazdeho radku
    for i in content {
        let _solution = solution::Solution::try_from(i).unwrap();
    }
    
    // simulated::run(100);
}
