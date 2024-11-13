#[derive(Debug)]
pub struct Deck {
    cards: Vec<String>
}

fn main() {
    // list of suites - hearts/spades
    let suites = vec!["Hearts", "Spades", "Diamonds"];
    //list of values - ace/two/three
    let values = vec!["Ace", "Two", "Three"];

    let mut cards = vec![];

    for suit in suites {
        for value in &values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    let deck: Deck = Deck{cards: cards};
    println!("Deck: {:?}", deck)
}
