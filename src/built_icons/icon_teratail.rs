use crate::SimpleIcon; pub struct IconTeratail; impl Default for IconTeratail { fn default() -> Self { IconTeratail } } impl SimpleIcon for IconTeratail { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>teratail</title><path d=\"M9.81.968h4.375L24 23.032h-5.107L12.121 6.605h-.198L5.148 23.03H0Z\"/></svg>" } }
