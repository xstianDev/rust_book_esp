use ch11_01_how_to_write_tests::Rectangle;

// Integration tests: testea la interfaz pública del proyecto
#[test]
fn test_can_hold() {
    let rect1 = Rectangle { w: 1, h: 1 };
    let rect2 = Rectangle { w: 2, h: 2 };
    assert!(rect2.can_hold(&rect1));
}