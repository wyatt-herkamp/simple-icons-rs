use crate::SimpleIcon;
pub struct Iconpeertube;
impl SimpleIcon for Iconpeertube {
    fn icon() -> &'static str {
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>PeerTube</title><path d=\"M12 6.545v10.91L20.727 12M3.273 12v12L12 17.455M3.273 0v12L12 6.545\"/></svg>"
    }
}
