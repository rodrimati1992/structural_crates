use super::derive_from_str;

use as_derive_utils::test_framework::Tests;



#[test]
fn test_cases(){
    Tests::load("structural_alias").run_test(derive_from_str);
}