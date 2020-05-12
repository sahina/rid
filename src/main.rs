mod utils;

use std::hash::Hash;

#[derive(Debug, Hash)]
struct Rid {
    name: String,
    parts: Vec<Part>,
}

impl Rid {
    fn new<S: Into<String>>(name: S) -> Self {
        Rid {
            name: name.into(),
            parts: Vec::new(),
        }
    }

    fn part(mut self, p: Part) -> Self {
        self.parts.push(p);

        self
    }

    fn calc_hash(&self) -> u64 {
        utils::calculate_hash(self)
    }
}

// impl Hash for Rid {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//         self.parts.hash(state);
//     }
// }

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Part {
    key: String,
    order: i16,
}

impl Part {
    fn new<S: Into<String>>(key: S) -> Self {
        Part {
            key: key.into(),
            order: 1,
        }
    }

    fn order(mut self, order: i16) -> Self {
        self.order = order;

        self
    }

    fn calc_hash(&self) -> u64 {
        utils::calculate_hash(self)
    }
}

fn main() {
    // rid:partition:service:region:account-id:resource-type/resource-id

    // API features
    //
    // template
    //      default

    //      default and change
    //      supply own template
    // view id
    // hash id
    // view template
    //      simple
    //      meta
    // view
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_api() {
        let rid = Rid::new("Email")
            .part(Part::new("rid").order(1))
            .part(Part::new("partition").order(2))
            .part(Part::new("service").order(3))
            .part(Part::new("region").order(4))
            .part(Part::new("account-id").order(5))
            .part(Part::new("resource-type").order(6))
            .part(Part::new("resource-id").order(7));

        println!("{:?}", rid);
    }

    #[test]
    fn hash_rid() {
        let rid_email = Rid::new("Email");
        let rid_url = Rid::new("Url");

        assert_ne!(rid_email.calc_hash(), rid_url.calc_hash());
    }
}
