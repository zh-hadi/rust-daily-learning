#[derive(Debug)]
struct Product {
    name: String,
    price: i32,
}

impl Product{
    fn new(name: String, price: i32)->Self{
        Self{name, price}
    }

    fn info(&self){
        println!("product name: {}\nPrice: {}", self.name, self.price)
    }

    fn s_print(&self){
        println!("{:?}", self);
    }

    fn change_name(&mut self, name: String){
        self.name = name;
    }
}

fn main(){
    let mut product1 = Product::new("pant".to_string(), 20);
    product1.s_print();
    product1.change_name("T-Shirt".to_string());
    product1.info();
    product1.s_print();
}