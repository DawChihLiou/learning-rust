// project: rectangles

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
    
fn main() {
    let scale = 2;
    let rect = Rectangle {
        // dbg macro takes ownership and returns ownership
        width: dbg!(30 * scale),
        height: 50,
    }; 
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    // {:#?} is to pretty-print
    println!("rect is {:#?}", rect);
    dbg!(&rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
