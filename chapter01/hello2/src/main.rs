fn main() {
    greet_world();
}

fn greet_world() {
    let southern_germany = "grüß gott";
    let english = "hello world";
    let regions = [southern_germany, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
