use crate::SimpleIcon;
pub struct Iconradar;
impl SimpleIcon for Iconradar {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Radar</title><path d=\"M12 0L2.197 23.975 12 19.952 21.803 24z\"/></svg>"
    }
}
