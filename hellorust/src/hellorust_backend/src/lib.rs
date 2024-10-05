#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
use std::cell::RefCell;
use ic_cdk::{query, update};

thread_local! {
    static STORE: RefCell<String> = RefCell::default();
}

#[query(name = "getMessage")]
fn get_message()-> String {
    STORE.with(|store| {
        store.borrow().clone()  // STOREに書き込まれた値を返却
    })
}

#[update(name = "setMessage")]
fn set_message(text: String) {
    STORE.with(|store| {
        *store.borrow_mut() = text; // 仮引数の値をSTOREに書き込む
    });
}