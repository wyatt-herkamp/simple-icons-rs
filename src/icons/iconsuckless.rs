use crate::SimpleIcon;
pub struct Iconsuckless;
impl SimpleIcon for Iconsuckless {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>suckless</title><path d=\"M0 4h24v4H4v2h20v10H0v-4h20v-2H0z\"/></svg>"
    }
}