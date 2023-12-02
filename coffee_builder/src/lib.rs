//! Ulangan Rust #4
//! Builder pattern dapat dibentuk dengan memanfaatkan `struct`.

/// # Coffee Builder
/// Program sederhana meracik Kopi
///
/// ```
/// use coffee_builder::{Coffee, CoffeeBuilder};
///
/// let coffee: Coffee = CoffeeBuilder::new().set_antoccino_coffee().build();
/// assert_eq!(
///     format!("{:?}", coffee),
///     "Coffee { sort: \"Americano\", milk: [Milk { fat: 0.5 }], sugar: [] }"
/// );
/// ```
#[derive(Debug)]
pub struct Coffee {
    pub sort: String,
    pub milk: Vec<Milk>,
    pub sugar: Vec<Sugar>,
}

#[derive(Debug)]
pub struct Milk {
    pub fat: f32,
}

#[derive(Debug)]
pub struct Sugar {
    pub category: String,
}

pub struct CoffeeBuilder {
    pub sort: String,
    pub milk: Vec<Milk>,
    pub sugar: Vec<Sugar>,
}

impl CoffeeBuilder {
    pub fn new() -> CoffeeBuilder {
        CoffeeBuilder {
            sort: String::new(),
            milk: Vec::new(),
            sugar: Vec::new(),
        }
    }

    pub fn set_black_coffee(mut self) -> CoffeeBuilder {
        self.sort = String::from("Black");
        self
    }

    pub fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        self.sort = String::from("Cubano");
        self.sugar.push(Sugar {
            category: String::from("Brown"),
        });
        self
    }

    pub fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        self.sort = String::from("Americano");
        self.milk.push(Milk { fat: 0.5 });
        self
    }

    pub fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        self.milk.push(Milk { fat });
        self
    }

    pub fn with_sugar(mut self, category: String) -> CoffeeBuilder {
        self.sugar.push(Sugar { category });
        self
    }

    pub fn build(self) -> Coffee {
        Coffee {
            sort: self.sort,
            milk: self.milk,
            sugar: self.sugar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        let coffee = CoffeeBuilder::new()
            .set_black_coffee()
            .with_sugar("Regular".to_string())
            .with_milk(3.2)
            .build();

        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Black\", milk: [Milk { fat: 3.2 }], sugar: [Sugar { category: \"Regular\" }] }"
        );

        let coffee: Coffee = CoffeeBuilder::new().set_antoccino_coffee().build();

        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Americano\", milk: [Milk { fat: 0.5 }], sugar: [] }"
        );

        let coffee = CoffeeBuilder::new().set_cubano_coffee().build();

        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Cubano\", milk: [], sugar: [Sugar { category: \"Brown\" }] }"
        );
    }
}
