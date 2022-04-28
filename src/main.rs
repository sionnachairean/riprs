use riprs::parser::ripscrip;

fn main() {
    println!(
        "{:?}",
        ripscrip("!|w00001B0M10|v00002E1M|*|e|E|g0509|H|>|c0A|Q000102030405060708090A0B0C0D0E0F|a051B|W00")
    )
}
