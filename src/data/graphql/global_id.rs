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

    bytes[0] = match node_type {
        Game => 0x1B,
        User => 0x53,
        Run => 0x47,
        Category => 0x0B,
        Level => 0x2F,
    };
    bytes[1] |= 0xE0;

    ID::from(base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD))
}

#[allow(unused)]
pub fn parse_global_id(
    global_id: &juniper::ID,
) -> Result<(u64, NodeType), Box<dyn std::error::Error>> {
    let mut bytes = base64::decode_config(&global_id.to_string(), base64::URL_SAFE_NO_PAD)
        .expect("infallible");
    assert!(bytes[1] == 0xE0, "second-high byte must be 0xE0");

    let node_type = match bytes[0] {
        0x1B => Game,
        0x53 => User,
        0x47 => Run,
        0x0B => Category,
        0x2F => Level,
        _ => panic!("high byte didn't match expected tag values"),
    };

    bytes[0] = 0;
    bytes[1] &= 0x0F;

    let mut bytes_array = [0u8; 8];
    let bytes = &bytes[..bytes_array.len()];
    bytes_array.copy_from_slice(bytes);

    Ok((u64::from_be_bytes(bytes_array), node_type))
}

#[test]
fn test_round_trip_global_ids() {
    use crate::utils::u64_from_base36;

    let cases = [
        (0x1234u64, NodeType::Game, "G-AAAAAAEjQ"),
        (0x0000_FFFF_FFFF_FFFF, NodeType::User, "U-D_______8"),
        (0x0000_FEDA_BCAC_EDAC, NodeType::Run, "R-D-2rys7aw"),
        (0x0, NodeType::Category, "C-AAAAAAAAA"),
        (0x1, NodeType::Level, "L-AAAAAAAAE"),
        (
            u64_from_base36("zzzzzzzz").expect("it's valid"),
            NodeType::Level,
            "l-ACkNdA__8",
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
