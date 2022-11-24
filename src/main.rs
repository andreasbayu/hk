use hk::held_karp;


fn main() {
    let array = vec![
        vec![0, 26, 37, 99],
        vec![26, 0, 55, 63],
        vec![37, 55, 0, 28],
        vec![99, 63, 28, 0]
        ];

    let data = held_karp(array);
    println!("{:?}", data)
}
