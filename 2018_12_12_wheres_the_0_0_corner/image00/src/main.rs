fn main() {
    println!("IMAGE CROP TEST");
    lets_see();
}

fn lets_see() {
    use image::GenericImageView;
    use image::DynamicImage;

    let mut buffer =
        image::open("resources/4x3.bmp")
            .unwrap();

    println!("BEFORE");
    let (x, y) = buffer.dimensions();
    println!("size: x={}, y={}", x, y);
    println!();

    println!("AFTER");
    let cropped = buffer.crop(0, 0, 3, 2);
    let (x, y) = cropped.dimensions();

    println!("size: x={}, y={}", x, y);
    println!();

    // NOTES:
    // From manual inspection it looks like the crop function uses
    // the top left corner as (x=0,y=0)

    cropped.save("target/out.bmp").unwrap();
}