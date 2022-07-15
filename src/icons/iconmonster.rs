use crate::SimpleIcon;
pub struct Iconmonster;
impl SimpleIcon for Iconmonster {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Monster</title><path d=\"M0 0V24H5.42V12.39L12 18.19L18.58 12.39V24H24V0L12 11.23L0 0Z\"/></svg>"
    }
}
