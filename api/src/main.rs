use models::page::PageBuilder;

fn main() {
    println!("Hello, world!");
    let page = PageBuilder::default()
        .text("Hello, world!".to_string())
        .build()
        .unwrap();
    println!("{:?}", page);
}
