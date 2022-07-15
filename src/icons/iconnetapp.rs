use crate::SimpleIcon;
pub struct Iconnetapp;
impl SimpleIcon for Iconnetapp {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>NetApp</title><path d=\"M0 2v20h9.33V10h5.34v12H24V2Z\"/></svg>"
    }
}