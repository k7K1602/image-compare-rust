use raster::compare;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let pciture_1_path = args.get(1).unwrap();
    let path_1_ref: &str = &pciture_1_path;

    let picture_2_path = args.get(2).unwrap();
    let path_2_ref: &str = &picture_2_path;

    let image1 = raster::open(path_1_ref).unwrap();
    let image2 = raster::open(path_2_ref).unwrap();

    let result = compare::equal(&image1, &image2).unwrap();
    if result {
        println!("Pictures are the same")
    } else {
        println!("Pictures are different")
    }
}
