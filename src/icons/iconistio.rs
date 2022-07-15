use crate::SimpleIcon;
pub struct Iconistio;
impl SimpleIcon for Iconistio {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Istio</title><path d=\"M4 21 20 21 10 24zM4 20 10 19 10 8zM11 19 20 20 11 0z\"/></svg>"
    }
}