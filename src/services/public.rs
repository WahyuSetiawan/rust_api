#[get("/")]
pub fn index() -> &'static str {
    "Hello this is public page"
}
