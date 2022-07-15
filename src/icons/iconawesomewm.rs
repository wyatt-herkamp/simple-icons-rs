use crate::SimpleIcon;
pub struct Iconawesomewm;
impl SimpleIcon for Iconawesomewm {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>awesomeWM</title><path d=\"M0 24V8.25h16.5V7.5H0V0h24v24h-7.5v-8.25h-9v.75h8.25V24z\"/></svg>"
    }
}
