#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Item {
    order: u8,
    name: String,
}

#[derive(Debug)]
struct Rid {
    items: Vec<Item>,
}

impl Rid {
    fn new() -> Self {
        Rid {
            items: vec![
                Item {
                    name: String::from("rid"),
                    order: 1,
                },
                Item {
                    name: String::from("partition"),
                    order: 2,
                },
                Item {
                    name: String::from("service"),
                    order: 3,
                },
                Item {
                    name: String::from("region"),
                    order: 4,
                },
                Item {
                    name: String::from("account_id"),
                    order: 5,
                },
                Item {
                    name: String::from("resource_type"),
                    order: 6,
                },
                Item {
                    name: String::from("resource_id"),
                    order: 7,
                },
            ],
        }
    }

    fn id(&mut self) -> String {
        let mut id = String::new();

        self.items.sort_by(|l, r| l.order.cmp(&r.order));
        self.items.iter().for_each(|i| id.push_str(i.name.as_str()));

        id
    }
}

fn main() {
    let mut email = Rid::new();
    println!("{:?}", email.id());
}
