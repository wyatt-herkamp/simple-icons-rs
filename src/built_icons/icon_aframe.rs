use crate::SimpleIcon; pub struct IconAframe; impl Default for IconAframe { fn default() -> Self { IconAframe } } impl SimpleIcon for IconAframe { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>A-Frame</title><path d=\"M17.37 17.07H6.57L4.24 24H3.01l8.23-24h1.52l8.23 24h-1.3zm-.39-1.13l-5-14.96-5.03 14.98h10.03Z\"/></svg>" } }