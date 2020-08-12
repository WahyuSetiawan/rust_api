mod public;

#[get("/")]
pub fn index()-> &'static str{
    return public::index();
}