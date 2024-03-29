use enum_impl::EnumImpl;

#[derive(EnumImpl)]
pub enum WebEvent {
    #[enum_impl(pub is)]
    PageLoad,
    #[enum_impl(pub is)]
    PageUnload,
    #[enum_impl(pub as_ref, as_ref_mut, impl from)]
    KeyPress(char),
    #[enum_impl(pub as_ref, as_ref_mut, pub into)]
    Paste(String),
    #[enum_impl(pub from = "click_from_coordinates", pub is, pub as_ref)]
    Click { x: i64, y: i64 },
}

fn main() {
    let page_load = WebEvent::PageLoad;
    assert!(page_load.is_page_load());

    let page_unload = WebEvent::PageUnload;
    assert!(page_unload.is_page_unload());

    let mut key_press = WebEvent::from('c');
    assert_eq!(*key_press.as_key_press().unwrap(), 'c');

    *key_press.as_key_press_mut().unwrap() = 'd';
    assert_eq!(*key_press.as_key_press().unwrap(), 'd');

    let paste = WebEvent::Paste("hello world".to_owned());
    assert_eq!(paste.as_paste().unwrap(), "hello world");
    assert_eq!(paste.into_paste().unwrap(), "hello world".to_owned());

    let click = WebEvent::click_from_coordinates(-10, 10);
    assert!(click.is_click());
    let WebEvent::Click { x, y } = click else { panic!() };
    assert_eq!(x, -10);
    assert_eq!(y, 10);
}
