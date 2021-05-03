use super::*;
#[test]
fn test_bsearch() {
    assert_eq!(-1, binary(&3, &[]));
    assert_eq!(-1, binary(&3, &[1]));
    assert_eq!(0, binary(&1, &[1]));
    ///
    assert_eq!(0, binary(&1, &[1, 3, 5]));
    assert_eq!(1, binary(&3, &[1, 3, 5]));
    assert_eq!(2, binary(&5, &[1, 3, 5]));
    assert_eq!(-1, binary(&0, &[1, 3, 5]));
    assert_eq!(-1, binary(&2, &[1, 3, 5]));
    assert_eq!(-1, binary(&4, &[1, 3, 5]));
    assert_eq!(-1, binary(&6, &[1, 3, 5]));
    ///
    assert_eq!(0, binary(&1, &[1, 3, 5, 7]));
    assert_eq!(1, binary(&3, &[1, 3, 5, 7]));
    assert_eq!(2, binary(&5, &[1, 3, 5, 7]));
    assert_eq!(3, binary(&7, &[1, 3, 5, 7]));
    assert_eq!(-1, binary(&0, &[1, 3, 5, 7]));
    assert_eq!(-1, binary(&2, &[1, 3, 5, 7]));
    assert_eq!(-1, binary(&4, &[1, 3, 5, 7]));
    assert_eq!(-1, binary(&6, &[1, 3, 5, 7]));
    assert_eq!(-1, binary(&8, &[1, 3, 5, 7]));
}
