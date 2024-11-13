#[derive(Debug)]
pub struct Deck {
    cards: Vec<String>
}

fn main() {
    let deck: Deck = Deck{cards: Vec::new()};
    println!("Deck: {:?}", deck)
}
