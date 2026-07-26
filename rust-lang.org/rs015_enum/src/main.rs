enum Ticket {
    BackStage(f32, String),
    Vip(f32, String),
    Regular(f32),
}
fn main() {
    let tickets = vec![
        Ticket::Regular(2.12),
        Ticket::BackStage(12.21, "Jaber".to_owned()),
        Ticket::Vip(191.21, "Anya".to_owned()),
        Ticket::Regular(2.12),
        Ticket::Vip(191.21, "Masum".to_owned()),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::BackStage(price, name) => {
                dbg!(price, name);
            }
            Ticket::Regular(price) => {
                dbg!(price);
            }
            Ticket::Vip(price, name) => {
                dbg!(price, name);
            }
        }
    }
}
