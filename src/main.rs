use gua64::Gua64;

fn main() {
    let gua64 = Gua64::new();

    let message = "hello，世界".as_bytes();
    println!("Original: {:?}", message);

    let encoded = gua64.encode(message);
    println!("Encoded: {}", encoded);

    let decoded = gua64.decode(&encoded);
    println!("Decoded: {:?}", String::from_utf8(decoded).unwrap());

    let is_valid = gua64.verify(&encoded);
    println!("Is valid: {}", is_valid);
}
