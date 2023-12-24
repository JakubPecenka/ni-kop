#[derive(Debug, Clone, Default)]
pub struct Solution {
    name: String,
    weight: i32,
    bitmap: Vec<i32>,
}

impl Solution {
    pub fn new(name: &str, weight: i32, bitmap: Vec<i32>) -> Self {
        Self {
            name: name.to_string(),
            weight,
            bitmap,
        }
    }    
}

// Optimální řešení by byl následující řádek v souboru wuf04-06-E-opt.dat
// uf20-01000 10282 -1 2 3 -4 5 6 7 8 9 10 -11 12 13 14 -15 16 -17 18 19 -20 0
impl TryFrom<&str> for Solution {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s = s.trim().split_whitespace().collect::<Vec<_>>();

        let name = s.remove(0).trim().to_string();
        let weight = s.remove(0).parse::<i32>().unwrap();
        let mut bitmap = s.iter()
            .map(|x| x.parse::<i32>().unwrap())
            .map(|x| if x < 0 { 0 } else { 1 } )
            .collect::<Vec<_>>();
        bitmap.pop();

        Ok(Self {
            name,
            weight,
            bitmap,
        })
    }    
}