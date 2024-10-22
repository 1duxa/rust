
#[allow(dead_code)]

pub fn reverse_str_iter(input: &str) -> String {
    input.chars().rev().collect()
}
#[allow(dead_code)]

pub fn reverse_str_for(input: &str) -> String {
    
    let mut res:String = "".to_string();

    for symbol in input.chars(){
        
        res.insert(0, symbol);

    }

    res
}
