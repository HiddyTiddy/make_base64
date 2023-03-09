use make_base64::length_required;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let size = 0x40000000;
    println!("converting {}", human_bytes::human_bytes(size as f64));

    let input = (0..size).map(|_| rng.gen()).collect::<Vec<u8>>();
    let mut buffer = Vec::new();
    buffer.resize(length_required(size), 0);
    let written = make_base64::base64(input, &mut buffer);
    println!("{:?}", written.map(|w| human_bytes::human_bytes(w as f64)));
}
