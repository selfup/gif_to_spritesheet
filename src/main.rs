use g2s::convert;

fn main() {
    let input = "player.gif";
    let output = "player.png";

    match convert(input, output) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    }
}
