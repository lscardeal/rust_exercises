#[derive(Debug)]
struct FinancialMarket {
    stocks: Vec<Stock>
}

#[derive(Debug)]
struct Stock {
    name: String,
    price: f32,
    price_history: Vec<f32>
}

struct StockUpdate {
    name: String,
    new_price: f32
}

impl FinancialMarket {
    fn new() -> Self {
        Self { stocks: vec![] }
    }

    fn add_stock(&mut self, stock: Stock) {
        self.stocks.push(stock);
    }

    fn get_stocks(&mut self) -> &mut Vec<Stock> {
        &mut self.stocks
    }
}

impl Stock {
    fn new(name: String, price: f32) -> Self {
        Self {
            name,
            price,
            price_history: vec![]
        }
    }

    fn price(&self) -> f32 {
        self.price
    }

    fn update_price(&mut self, new_price: f32) {
        self.price_history.push(self.price);
        self.price = new_price;
    }

    fn price_history(&self) -> &Vec<f32> {
        &self.price_history
    }
}

impl StockUpdate {
    fn new(name: String, new_price: f32) -> Self {
        Self { name, new_price }
    }
}

fn main() {
    let mut financial_market: FinancialMarket = FinancialMarket::new();
    let mut abx_stock: Stock = Stock::new(String::from("Abx"), 14.5);
    let mut xxw_stock: Stock = Stock::new(String::from("XXW"), 7.42);
    let mut bat_stock: Stock = Stock::new(String::from("Bat"), 21.17);

    financial_market.add_stock(abx_stock);
    financial_market.add_stock(xxw_stock);
    financial_market.add_stock(bat_stock);

    let mut updates: Vec<StockUpdate> = vec![];
    updates.push(StockUpdate::new(String::from("Bat"), 20.97));
    updates.push(StockUpdate::new(String::from("Abx"), 15.05));
    updates.push(StockUpdate::new(String::from("XXW"), 7.43));

    for u in updates {
        if let Some(stock) = financial_market.get_stocks().iter_mut().find(|s| s.name == u.name) {
            stock.update_price(u.new_price);
        }
    }

    println!("{:#?}", financial_market);
}