#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn world(name: String) -> String {
    format!("World, {}!", name)
}

// Enable Candid export
ic_cdk::export_candid!();

