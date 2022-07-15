use crate::SimpleIcon;
pub struct Iconhouzz;
impl SimpleIcon for Iconhouzz {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Houzz</title><path d=\"M1.27 0V24H9.32V16.44H14.68V24H22.73V10.37L6.61 5.75V0H1.27Z\"/></svg>"
    }
}