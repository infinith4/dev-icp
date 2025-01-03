#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[ic_cdk::query]
fn call(req: String) -> String {
    format!("req: {}", req)
}

// Enable Candid export
ic_cdk::export_candid!();