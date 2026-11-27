use std::collections::VecDeque;


enum Algorithm {
    Optimal,
    Fifo,
    Lru
}


fn simulate(algorithm: Algorithm, initial_frames: &Vec<Option<i32>>, reference: &[i32]) -> String {
    let mut frames = initial_frames.clone();
    let mut result = String::new();
    let mut time = 0;
    let mut queue: Vec<Option<i32>> = frames.iter().filter(|f| {f.is_some()}).copied().collect();

    result += &format!("{:<12} | {:<12} | {:<12} |", "Time", "RS", "Hit/Miss");
    for frame in 0..initial_frames.len() {
        result += &format!(" F{:<11} |", frame)
    }
    result += &format!(" {:<12}\n", "Replacement? (who/why)");

    while time < reference.len() {
        let page_to_load = reference[time];
        let mut fault = false;
        let mut notes = String::new();
        let mut page_in_frame = false;
        let mut empty_frame: Option<usize> = None;

        for (position, frame) in frames.iter().enumerate() {
            match frame {
                // There is a page in this frame
                Some(page) => {
                    // We have the page loaded
                    if *page == page_to_load {
                        notes = "page in frame".to_string();
                        page_in_frame = true;
                    }  
                }
                // Empty frame
                None => {
                    match empty_frame {
                        Some(_) => {}
                        None => {
                            empty_frame = Some(position)
                        }
                    }
                }
            }
        }

        // We need to put the page in a frame
        if !page_in_frame {
            match empty_frame {
                // There is an empty frame we an load into
                Some(pos) => {
                    frames[pos] = Some(page_to_load);
                    notes = format!("{} loaded into F{}", page_to_load, pos)
                }
                // There is no empty frames, we must replace a page
                None => {
                    let f: usize;
                    fault = true;
                    match algorithm {
                        Algorithm::Optimal => {
                            let future = &reference[time+1..reference.len()];
                            let next_used: Vec<usize> = frames.iter().map(|p| {
                                if future.contains(&p.unwrap()) { 
                                    future.iter().position(|&i| { 
                                        i == p.unwrap()
                                    }).unwrap()
                                    } 
                                else {
                                    usize::MAX
                                }
                            }).collect();
                            f = next_used
                                .iter()
                                .enumerate()
                                .max_by_key(|&(_idx, val)| val)
                                .map(|(idx, _val)| idx)
                                .unwrap();

                        }
                        Algorithm::Fifo => {
                            f = frames.iter().position(|&i| i == queue[0]).unwrap();
                        }
                        Algorithm::Lru => {
                            f = 0
                        }
                    }
                    let victim = frames[f];
                    if let Some(pos) = queue.iter().position(|x| *x == victim) {
                        queue.remove(pos);
                    }
                    frames[f] = Some(page_to_load);
                    queue.push(Some(page_to_load));
                    notes = format!("{} replaced with {}", victim.unwrap(), page_to_load)

                }
            }
        }

        
        result += &format!("{:<12} | {:<12} | {:<12} |", time, page_to_load, if fault {"Miss"} else {"Hit"});
        for frame in 0..frames.len() {
            match frames[frame] {
                Some(e) => {result += &format!(" {:<12} |", e)},
                None => {result += &format!(" {:<12} |", "∅")}
            }
            
        }
    result += &format!(" {:<12}\n", notes);
        time += 1;
    }

    result
}


fn main() {
    let reference = [0, 1, 4, 0, 2, 3, 0, 1, 0, 2, 3, 4, 2, 3];
    let frames = vec![Some(0), Some(3), None, None];


    println!("OPT:\n{}\n", simulate(Algorithm::Optimal, &frames, &reference));
    println!("FIFO:\n{}\n", simulate(Algorithm::Fifo, &frames, &reference));
    println!("LRU:\n{}\n", simulate(Algorithm::Lru, &frames, &reference));
}
