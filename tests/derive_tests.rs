use std::borrow::Cow;

#[macro_use]
extern crate speedy_derive;
extern crate speedy;

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedStruct {
    a: u8,
    b: u16,
    c: u32
}

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedTupleStruct( u8, u16, u32 );

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedUnitStruct;

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedEmptyStruct {}

#[derive(PartialEq, Debug, Readable, Writable)]
enum DerivedSimpleEnum {
    A,
    B = 10,
    C
}

#[derive(PartialEq, Debug, Readable, Writable)]
enum DerivedEnum {
    A,
    B( u8, u16, u32 ),
    C {
        a: u8,
        b: u16,
        c: u32
    }
}

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedStructWithLifetime< 'a > {
    bytes: Cow< 'a, [u8] >
}

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedStructWithGenericRef< 'a, T: 'a + ?Sized > {
    inner: &'a T
}

#[derive(PartialEq, Debug, Readable, Writable)]
struct DerivedStructWithGeneric< T > {
    inner: T
}

macro_rules! define_test {
    ($($name:ident: $value:expr, $serialized:expr)*) => { $(
        #[test]
        fn $name() {
            use speedy::{
                Readable,
                Writable,
                Endianness
            };

            let original = $value;
            let serialized = original.write_to_vec( Endianness::LittleEndian ).unwrap();
            assert_eq!( serialized, $serialized );

            let deserialized = Readable::read_from_buffer( Endianness::LittleEndian, &serialized ).unwrap();
            assert_eq!( original, deserialized );
        }
    )* }
}

define_test!(
    test_derived_struct:
        DerivedStruct { a: 1, b: 2, c: 3 },
        &[1, 2, 0, 3, 0, 0, 0]

    test_derived_tuple_struct:
        DerivedTupleStruct( 1, 2, 3 ),
        &[1, 2, 0, 3, 0, 0, 0]

    test_derived_unit_struct:
        DerivedUnitStruct,
        &[]

    test_derived_empty_struct:
        DerivedEmptyStruct {},
        &[]

    test_derived_simple_enum_a:
        DerivedSimpleEnum::A,
        &[0, 0, 0, 0]

    test_derived_simple_enum_b:
        DerivedSimpleEnum::B,
        &[10, 0, 0, 0]

    test_derived_simple_enum_c:
        DerivedSimpleEnum::C,
        &[11, 0, 0, 0]

    test_derived_enum_unit_variant:
        DerivedEnum::A,
        &[0, 0, 0, 0]

    test_derived_enum_tuple_variant:
        DerivedEnum::B( 10, 20, 30 ),
        &[1, 0, 0, 0, 10, 20, 0, 30, 0, 0, 0]

    test_derived_enum_struct_variant:
        DerivedEnum::C { a: 100, b: 200, c: 300 },
        &[2, 0, 0, 0, 100, 200, 0, 44, 1, 0, 0]

    test_derived_struct_with_lifetime:
        DerivedStructWithLifetime { bytes: Cow::Borrowed( &[2, 4, 8] ) },
        &[3, 0, 0, 0, 2, 4, 8]

    test_derived_struct_with_generic:
        DerivedStructWithGeneric { inner: Cow::Borrowed( &[1_u8, 2_u8, 3_u8][..] ) },
        &[3, 0, 0, 0, 1, 2, 3]
);
