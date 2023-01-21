pub fn route(x: usize, input: &mut [u8]) -> Option<&[u8]> {
    match x {
        0 => Some(pass_through(input)),
        1 => Some(times_two(input)),
        2 => Some(div_two(input)),
        3 => Some(plus_one(input)),
        4 => Some(sub_one(input)),
        _ => None,
    }
}
pub fn pass_through(input: &mut [u8]) -> &[u8] {
    input
}
pub fn times_two(input: &mut [u8]) -> &[u8] {
    for x in input.iter_mut() {
        *x *= 2;
    }
    input
}
pub fn div_two(input: &mut [u8]) -> &[u8] {
    for x in input.iter_mut() {
        *x /= 2;
    }
    input
}
pub fn plus_one(input: &mut [u8]) -> &[u8] {
    for x in input.iter_mut() {
        *x += 1;
    }
    input
}
pub fn sub_one(input: &mut [u8]) -> &[u8] {
    for x in input.iter_mut() {
        *x -= 1;
    }
    input
}
