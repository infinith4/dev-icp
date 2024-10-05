#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn send(msg: String) -> String {
    format!("msg: {}", msg)
}
