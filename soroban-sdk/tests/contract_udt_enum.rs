#![cfg(feature = "testutils")]

use soroban_sdk::{
    contractimpl, contracttype, vec, BytesN, ConversionError, Env, IntoVal, Symbol, TryFromVal,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Udt {
    Aaa,
    Bbb(i32),
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

    let a = Udt::Aaa;
    let b = Udt::Bbb(3);
    let c = add::invoke(&env, &contract_id, &a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a vec will decode to a Udt if the first element is the
    // variant name as a Symbol, and following elements are tuple-like values
    // for the variant.
    let vec = vec![&env, Symbol::from_str("Aaa").into_val(&env)].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Ok(Udt::Aaa));
    let vec = vec![&env, Symbol::from_str("Bbb").into_val(&env), 8.into()].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Ok(Udt::Bbb(8)));

    // If an enum has a tuple like variant with one value, but the vec has
    // multiple values, it is an error. It is an error because decoding and
    // encoding will not round trip the data, and therefore partial decoding is
    // relatively difficult to use safely.
    let vec = vec![&env, Symbol::from_str("Aaa").into_val(&env), 8.into()].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Err(ConversionError));
    let vec = vec![
        &env,
        Symbol::from_str("Bbb").into_val(&env),
        8.into(),
        9.into(),
    ]
    .to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Err(ConversionError));
}
