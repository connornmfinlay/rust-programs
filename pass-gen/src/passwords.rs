
#[passwords::PasswordGenerator]
fn main() { 

let &pg = PasswordGenerator{
    length: 8,
    numbers: true,
    lowercase_letters: true,
    uppercase_letters: true,
    symbols: true,
    exclude_similar_characters: true,
    strict: true,

println!("{}", pg: pg.generate_one().unwrap());
}}


