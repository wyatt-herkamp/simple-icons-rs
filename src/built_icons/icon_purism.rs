use crate::SimpleIcon; pub struct IconPurism; impl Default for IconPurism { fn default() -> Self { IconPurism } } impl SimpleIcon for IconPurism { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Purism</title><path d=\"M24 19.588H0V4.412h24zM2.824 16.765h18.352v-9.53H2.824Z\"/></svg>" } }
