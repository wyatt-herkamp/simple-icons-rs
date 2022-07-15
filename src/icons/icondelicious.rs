use crate::SimpleIcon;
pub struct Icondelicious;
impl SimpleIcon for Icondelicious {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>del.icio.us</title><path d=\"M12 12H0v12h12V12zM24 0H12v12h12V0z\"/></svg>"
    }
}
