use super::*;

#[test]
fn bx() {
    let x = 0b01011111_u8;
    assert!(!x.b7());
    assert!(x.b6());
    assert!(!x.b5());
    assert!(x.b4());
    assert!(x.b3());
    assert!(x.b2());
    assert!(x.b1());
    assert!(x.b0());
}

#[test]
fn set_bx() {
    let mut x = 0u8;
    x.set_b0();
    assert_eq!(x, 0b00000001);
    x = 0;
    x.set_b1();
    assert_eq!(x, 0b00000010);
    x = 0;
    x.set_b2();
    assert_eq!(x, 0b00000100);
    x = 0;
    x.set_b3();
    assert_eq!(x, 0b00001000);
    x = 0;
    x.set_b4();
    assert_eq!(x, 0b00010000);
    x = 0;
    x.set_b5();
    assert_eq!(x, 0b00100000);
    x = 0;
    x.set_b6();
    assert_eq!(x, 0b01000000);
    x = 0;
    x.set_b7();
    assert_eq!(x, 0b10000000);
}

#[test]
fn reset_bx() {
    let mut x = !0u8;
    x.reset_b0();
    assert_eq!(x, !0b00000001);
    x = !0;
    x.reset_b1();
    assert_eq!(x, !0b00000010);
    x = !0;
    x.reset_b2();
    assert_eq!(x, !0b00000100);
    x = !0;
    x.reset_b3();
    assert_eq!(x, !0b00001000);
    x = !0;
    x.reset_b4();
    assert_eq!(x, !0b00010000);
    x = !0;
    x.reset_b5();
    assert_eq!(x, !0b00100000);
    x = !0;
    x.reset_b6();
    assert_eq!(x, !0b01000000);
    x = !0;
    x.reset_b7();
    assert_eq!(x, !0b10000000);
}

#[test]
fn toggle_bx() {
    let mut x = 0u8;
    x.toggle_b0();
    assert_eq!(x, 0b00000001);
    x = 0;
    x.toggle_b1();
    assert_eq!(x, 0b00000010);
    x = 0;
    x.toggle_b2();
    assert_eq!(x, 0b00000100);
    x = 0;
    x.toggle_b3();
    assert_eq!(x, 0b00001000);
    x = 0;
    x.toggle_b4();
    assert_eq!(x, 0b00010000);
    x = 0;
    x.toggle_b5();
    assert_eq!(x, 0b00100000);
    x = 0;
    x.toggle_b6();
    assert_eq!(x, 0b01000000);
    x = 0;
    x.toggle_b7();
    assert_eq!(x, 0b10000000);
    x = 0;

    x.toggle_b0();
    x.toggle_b0();
    assert!(x == 0);
    x.toggle_b1();
    x.toggle_b1();
    assert!(x == 0);
    x.toggle_b2();
    x.toggle_b2();
    assert!(x == 0);
    x.toggle_b3();
    x.toggle_b3();
    assert!(x == 0);
    x.toggle_b4();
    x.toggle_b4();
    assert!(x == 0);
    x.toggle_b5();
    x.toggle_b5();
    assert!(x == 0);
    x.toggle_b6();
    x.toggle_b6();
    assert!(x == 0);
    x.toggle_b7();
    x.toggle_b7();
    assert!(x == 0);
}
