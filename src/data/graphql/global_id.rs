#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeType {
    Game,
    User,
    Run,
    Category,
    Level,
}

use NodeType::*;

pub fn global_id(id: u64, node_type: NodeType) -> String {
    let mut bytes = id.to_be_bytes();
    assert!(bytes[0] == 0, "high byte of id must be zero");
    assert!(bytes[1] == 0, "second-high byte of id must be zero");

    bytes[0] = match node_type {
        Game => 0x18,
        User => 0x50,
        Run => 0x44,
        Category => 0x08,
        Level => 0x2C,
    };

    base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
}

pub fn parse_global_id(global_id: &str) -> (u64, NodeType) {
    let mut bytes = base64::decode_config(global_id, base64::URL_SAFE_NO_PAD).unwrap();
    assert!(bytes[1] == 0, "second-high byte must be zero");

    let node_type = match bytes[0] {
        0x18 => Game,
        0x50 => User,
        0x44 => Run,
        0x08 => Category,
        0x2C => Level,
        _ => panic!("high byte didn't match expected tag values"),
    };

    bytes[0] = 0;

    let mut bytes_array = [0u8; 8];
    let bytes = &bytes[..bytes_array.len()];
    bytes_array.copy_from_slice(bytes);

    (u64::from_be_bytes(bytes_array), node_type)
}

#[test]
fn test_round_trip_global_ids() {
    let cases = [
        (0x1234u64, NodeType::Game, "GAAAAAAAEjQ"),
        (0x0000_FFFF_FFFF_FFFF, NodeType::User, "UAD_______8"),
        (0x0000_FEDA_BCAC_EDAC, NodeType::Run, "RAD-2rys7aw"),
        (0x0, NodeType::Category, "CAAAAAAAAAA"),
        (0x1, NodeType::Level, "LAAAAAAAAAE"),
    ];

    for (id, node_type, global) in &cases {
        let global2 = global_id(*id, *node_type);
        assert_eq!(*global, global2);
        let (id2, node_type2) = parse_global_id(&global);
        assert_eq!(*id, id2);
        assert_eq!(*node_type, node_type2);
    }
}
