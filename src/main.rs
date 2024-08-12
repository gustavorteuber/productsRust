// Exemplificacao Rust - Estruturas e Enums

struct Product {
    name: String,
    price: f64,
}

// Implementando métodos para a estrutura Product
impl Product {
    fn new(name: &str, price: f64) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }

    fn display(&self) {
        println!("Product: {} - Price: ${:.2}", self.name, self.price);
    }
}

// Definindo um enum para representar o status de um pedido
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
}

// Implementando o display para o enum OrderStatus
impl OrderStatus {
    fn display(&self) {
        match self {
            OrderStatus::Pending => println!("Status: Pending"),
            OrderStatus::Shipped => println!("Status: Shipped"),
            OrderStatus::Delivered => println!("Status: Delivered"),
        }
    }
}

// Definindo uma estrutura para representar um Pedido
struct Order {
    id: u32,
    products: Vec<Product>,
    status: OrderStatus,
}

// Implementando métodos para a estrutura Order
impl Order {
    fn new(id: u32) -> Self {
        Self {
            id,
            products: Vec::new(),
            status: OrderStatus::Pending,
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn total_price(&self) -> f64 {
        self.products.iter().map(|p| p.price).sum()
    }

    fn display(&self) {
        println!("Order ID: {}", self.id);
        self.status.display();
        for product in &self.products {
            product.display();
        }
        println!("Total Price: ${:.2}", self.total_price());
    }

    fn ship(&mut self) {
        self.status = OrderStatus::Shipped;
    }

    fn deliver(&mut self) {
        self.status = OrderStatus::Delivered;
    }
}

// Função principal
fn main() {
    // Criando produtos
    let product1 = Product::new("Laptop", 1500.0);
    let product2 = Product::new("Smartphone", 800.0);

    // Criando um pedido e adicionando produtos
    let mut order = Order::new(1);
    order.add_product(product1);
    order.add_product(product2);

    // Exibindo o pedido
    println!("Initial Order Details:");
    order.display();

    // Enviando o pedido
    order.ship();
    println!("\nOrder Shipped:");
    order.display();

    // Entregando o pedido
    order.deliver();
    println!("\nOrder Delivered:");
    order.display();
}
