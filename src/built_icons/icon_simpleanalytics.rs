use crate::SimpleIcon; pub struct IconSimpleanalytics; impl Default for IconSimpleanalytics { fn default() -> Self { IconSimpleanalytics } } impl SimpleIcon for IconSimpleanalytics { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Simple Analytics</title><path d=\"M1.019 13.019h3.849V24h-3.85zm8.943-6.68h3.85V24h-3.85zM19.132 0h3.85v24h-3.85z\"/></svg>" } }
