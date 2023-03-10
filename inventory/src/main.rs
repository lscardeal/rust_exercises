#[derive(Clone, Debug)]
struct Product {
    name: String,
    price: f64,
    quantity: u32
}

struct Inventory {
    products: Vec<Product>
}

impl Product {
    fn update_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }

    fn print(&self) {
        println!("{:#?}", self);
    }
}

impl Inventory {
    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn remove_product(&mut self, name: &str) {
        let index = self.products.iter().position(|p| p.name == name).unwrap();
        self.products.remove(index);
    }

    fn remove_all_products_by_name(&mut self, name: &str) {
        self.products.retain(|product| product.name != name);
    }

    fn update_quantity(&mut self, name: &str, quantity: u32) {
        if let Some(product) = self.products.iter_mut().find(|p| p.name == name) {
            product.update_quantity(quantity);
        }
    }

    fn list_products(&self) {
        for produt in &self.products {
            produt.print();
            println!(" --- ");
        }
    }
}

fn main() {
    let banana: Product = Product { name: String::from("Banana"), price: 10.50, quantity: 100 };
    let onion: Product = Product { name: String::from("Onion"), price: 4.30, quantity: 100 };
    let mut inventory: Inventory = Inventory { products: vec![] };

    inventory.add_product(banana);
    inventory.add_product(onion);
    inventory.remove_product("Banana");
    inventory.update_quantity("Onion", 150);
    inventory.list_products();

}