use crate::SimpleIcon;
pub struct Iconframer;
impl SimpleIcon for Iconframer {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Framer</title><path d=\"M4 0h16v8h-8zM4 8h8l8 8H4zM4 16h8v8z\"/></svg>"
    }
}
