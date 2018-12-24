#![cfg(test)]

#[macro_use]
extern crate is_variant;

#[test]
fn it_works() {
    #[derive(IsVariant)]
    enum Foo<'a, 'b: 'a, T = String> {
        BarUnit,
        BazNewtype(T),
        QuxTuple(T, &'a T),
        LolStruct {
            field: &'b T,
        }
    }

    let unit = Foo::BarUnit::<Vec<u8>>;
    let newtype = Foo::BazNewtype(String::new());
    let tuple = Foo::QuxTuple(-42, &42);
    let structure = Foo::LolStruct { field: &3.14 };

    assert!(unit.is_bar_unit());
    assert!(!unit.is_baz_newtype());
    assert!(!unit.is_qux_tuple());
    assert!(!unit.is_lol_struct());

    assert!(!newtype.is_bar_unit());
    assert!(newtype.is_baz_newtype());
    assert!(!newtype.is_qux_tuple());
    assert!(!newtype.is_lol_struct());

    assert!(!tuple.is_bar_unit());
    assert!(!tuple.is_baz_newtype());
    assert!(tuple.is_qux_tuple());
    assert!(!tuple.is_lol_struct());

    assert!(!structure.is_bar_unit());
    assert!(!structure.is_baz_newtype());
    assert!(!structure.is_qux_tuple());
    assert!(structure.is_lol_struct());
}
