#![feature(proc_macro_hygiene)]
use annotation_rs::get_annotation;

use annotation_rs_test_macros::FullDerive;
use float_cmp::approx_eq;
use objects::attributes::Full;
use objects::enums::TestEnum;

#[derive(FullDerive)]
#[Full(
    object(i32 = 1, u16 = 2, float = 1.1, string = "test", enum2 = "aaa"),
    vector("1", "2", "3"),
    map(a = "aaa", b = "variant_b", c = "variant_c"),
    map2(a("a"), b("b"), c("c")),
    map3(a("aaa", "variant_b"), b("aaa"))
)]
struct Test;

#[test]
pub fn test_full() {
    let attr: Full = get_annotation!(Test, Full).unwrap();
    let simple = attr.object;
    assert_eq!(simple.int32, 1);
    assert_eq!(simple.unsigned16, 2);
    let float = simple.float;
    assert!(approx_eq!(f32, float, 1.1));
    assert_eq!(simple.string, "test");
    assert_eq!(simple.enum1, Some(TestEnum::VariantC));
    assert_eq!(simple.enum2, TestEnum::VariantA);
    assert_eq!(attr.vector, ["1", "2", "3"]);
    assert_eq!(attr.map["a"], TestEnum::VariantA);
    assert_eq!(attr.map["b"], TestEnum::VariantB);
    assert_eq!(attr.map["c"], TestEnum::VariantC);
}
