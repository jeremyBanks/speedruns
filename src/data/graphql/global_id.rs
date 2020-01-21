use juniper::ID;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeType {
    Game,
    User,
    Run,
    Category,
    Level,
}

use NodeType::*;

pub fn global_id(id: u64, node_type: NodeType) -> ID {
    let mut bytes = id.to_be_bytes();
    assert!(bytes[0] == 0, "high byte of id must be zero");
    assert!(bytes[1] == 0, "second-high byte of id must be zero");

    /*
     We want to encode the type and the original speedrun.com ID in a single
     64-bit integer, which we then base-64-encode per graphql conventions.

     speedrun.com IDs are 8 base-36 digits, which means they require
     log2(36**8) = ~41.3 bits, giving us 64 - 42 = 22 bits clear to work with.
     We only really need a few to identify our five node types, but since
     22 bits / 6 bits per base-64 character = ~3.6, we can instead use 18 of them
     to pick three meaningful prefix character in the encoded value.
    */

    let (a, b, c) = match node_type {
        Game => (0b1000_0001, 0b1010_1001, 0b1000_0000), // gam
        User => (0b1011_1010, 0b1100_1010, 0b1100_0000), // usr
        Run => (0b1010_1110, 0b1110_1001, 0b1100_0000),  // run
        Category => (0b0111_0001, 0b1010_1011, 0b0100_0000), // cat
        Level => (0b1001_0110, 0b1111_1001, 0b0100_0000), // lvl
    };

    bytes[0] = a;
    bytes[1] = b;
    bytes[2] = (c & 0b1100_0000) | (bytes[2] & 0b0011_1111);

    ID::from(base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD))
}

pub fn parse_global_id(
    global_id: &juniper::ID,
) -> Result<(u64, NodeType), Box<dyn std::error::Error>> {
    let mut bytes = base64::decode_config(&global_id.to_string(), base64::URL_SAFE_NO_PAD)
        .expect("infallible");

    let node_type = match bytes[0] {
        0b1000_0001 => Game,
        0b1011_1010 => User,
        0b1010_1110 => Run,
        0b0111_0001 => Category,
        0b1001_0110 => Level,
        _ => panic!("high byte didn't match expected tag values"),
    };

    // clear tag bits
    bytes[0] = 0;
    bytes[1] = 0;
    bytes[2] &= 0b0011_1111;

    let mut bytes_array = [0u8; 8];
    let bytes = &bytes[..bytes_array.len()];
    bytes_array.copy_from_slice(bytes);

    Ok((u64::from_be_bytes(bytes_array), node_type))
}

#[test]
fn test_round_trip_global_ids() {
    use crate::utils::u64_from_base36;

    let cases = [
        (0x1234u64, NodeType::Game, "gamAAAAAEjQ"),
        (0x0, NodeType::Category, "catAAAAAAAA"),
        (0x1, NodeType::Level, "lvlAAAAAAAE"),
        (
            u64_from_base36("zzzzzzzz").expect("it's valid"),
            NodeType::Level,
            "lvlCkNdA__8",
        ),
        (
            u64_from_base36("0abcdefg").expect("it's valid"),
            NodeType::Level,
            "lvlABTpYwkw",
        ),
    ];

    for (id, node_type, global) in &cases {
        let global2 = global_id(*id, *node_type);
        assert_eq!(**global, *global2);
        let (id2, node_type2) =
            parse_global_id(&ID::from((*global).to_string())).expect("to be valid");
        assert_eq!(*id, id2);
        assert_eq!(*node_type, node_type2);
    }
}
