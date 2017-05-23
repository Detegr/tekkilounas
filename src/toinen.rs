use std::thread;
use std::default::Default;
use std::ops::Add;

pub fn sum_elements<T>(arr: &'static [T], n: usize) -> Vec<T>
    where T: Add<Output = T> + Default + Send + Sync + Clone
{
    let mut ret = vec![];
    let mut thread_handles = vec![];
    for chunk in arr.chunks(n) {
        thread_handles.push(thread::spawn(move || {
            chunk.iter().fold(Default::default(), |acc: T, elem| acc + elem.clone())
        }));
    }

    for thread in thread_handles {
        match thread.join() {
            Ok(value) => ret.push(value),
            Err(e) => panic!("{:?}", e),
        }
    }
    ret
}
