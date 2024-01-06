
pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    // Kata - https://www.codewars.com/kata/64fbfe2618692c2018ebbddb
    let mut flick_val: bool = true;
    let mut return_list: Vec<bool> = Vec::new();
    for i in list {
        if i.eq(&"flick") {
            flick_val = !flick_val;
        }
        return_list.push(flick_val)
    }
    return return_list
}
