

// Option<i32> allows me to say whether there is or is not a value in each frame.
// match frames[x] allows me to check if frame x has a page associated with it.

fn optimal(initial_frames: &[Option<i32>], reference: &[i32]) -> String{
    let result = String::new();

    result
}    

fn fifo(initial_frames: &[Option<i32>], reference: &[i32]) -> String{
    let result = String::new();

    result
}    

fn lru(initial_frames: &[Option<i32>], reference: &[i32]) -> String{
    let result = String::new();

    result
}    

fn main() {
    let reference = [0, 1, 4, 0, 2, 3, 0, 1, 0, 2, 3, 4, 2, 3];
    let frames = [Some(0), Some(3), None, None];


    println!("OPT:\n{}\n", optimal(&frames, &reference));
    println!("FIFO:\n{}\n", fifo(&frames, &reference));
    println!("LRU:\n{}\n", lru(&frames, &reference));
}
