use crate::SimpleIcon;
pub struct Iconplex;
impl SimpleIcon for Iconplex {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Plex</title><path d=\"M11.643 0H4.68l7.679 12L4.68 24h6.963l7.677-12-7.677-12\"/></svg>"
    }
}
