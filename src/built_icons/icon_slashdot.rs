use crate::SimpleIcon; pub struct IconSlashdot; impl Default for IconSlashdot { fn default() -> Self { IconSlashdot } } impl SimpleIcon for IconSlashdot { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Slashdot</title><path d=\"M19.777 0L7.037 24H1.868L14.605 0h5.172zm2.354 19.801c0 2.268-1.841 4.105-4.109 4.105s-4.107-1.838-4.107-4.105 1.839-4.107 4.107-4.107 4.109 1.839 4.109 4.107z\"/></svg>" } }
