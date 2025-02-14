#![cfg(feature = "testutils")]

use std::io::Cursor;

use soroban_sdk::{
    contractimpl, contracttype, map, BytesN, ConversionError, Env, Symbol, TryFromVal,
};
use stellar_xdr::{
    ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeTuple,
    ScSpecTypeUdt,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt {
    pub a: i32,
    pub b: i32,
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, Contract);

    let a = Udt { a: 5, b: 7 };
    let b = Udt { a: 10, b: 14 };
    let c = add::invoke(&env, &contract_id, &a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a map will decode to a Udt if the symbol keys match the
    // fields.
    let map = map![&env, (Symbol::from_str("a"), 5), (Symbol::from_str("b"), 7)].to_raw();
    let udt = Udt::try_from_val(&env, map);
    assert_eq!(udt, Ok(Udt { a: 5, b: 7 }));

    // If a struct has fields a, b, and a map is decoded into it where the map
    // has fields a, b, and c, it is an error. It is an error because decoding
    // and encoding will not round trip the data, and therefore partial decoding
    // is relatively difficult to use safely.
    let map = map![
        &env,
        (Symbol::from_str("a"), 5),
        (Symbol::from_str("b"), 7),
        (Symbol::from_str("c"), 9)
    ]
    .to_raw();
    let udt = Udt::try_from_val(&env, map);
    assert_eq!(udt, Err(ConversionError));
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::read_xdr(&mut Cursor::new(&__SPEC_XDR_ADD)).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        name: "add".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
            ScSpecFunctionInputV0 {
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        }))]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);
}
