use training_mod_tui_2::Toggle;

#[test]
fn toggle_serialize() {
    let t = Toggle {
        title: "Title",
        value: 5,
        max: 10,
    };
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json, "5");
}

#[test]
fn toggle_increment() {
    let mut t = Toggle {
        title: "Title",
        value: 5,
        max: 10,
    };
    t.increment();
    assert_eq!(t.value, 6);
    t.value = 9;
    t.increment();
    assert_eq!(t.value, 10);
    t.increment();
    assert_eq!(t.value, 0);
}

#[test]
fn toggle_decrement() {
    let mut t = Toggle {
        title: "Title",
        value: 5,
        max: 10,
    };
    t.decrement();
    assert_eq!(t.value, 4);
    t.value = 1;
    t.decrement();
    assert_eq!(t.value, 0);
    t.decrement();
    assert_eq!(t.value, 10);
}
