use crate::SimpleIcon; pub struct IconInfiniti; impl Default for IconInfiniti { fn default() -> Self { IconInfiniti } } impl SimpleIcon for IconInfiniti { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Infiniti</title><path d=\"M6.893 14.606C5.17 14.46 2.18 13.33 2.18 11.359c0-2.5 4.343-4.818 9.819-4.818 5.75 0 9.82 2.318 9.82 4.818 0 1.97-2.978 3.087-4.702 3.233-.475-.609-5.118-6.791-5.118-6.791s-4.662 6.232-5.106 6.805zm13.744 2.115C22.921 15.6 24 13.734 24 12.088c0-3.533-4.928-6.264-12.001-6.264C4.927 5.824 0 8.555 0 12.088c0 1.646 1.079 3.511 3.363 4.633 2.118 1.041 5.116 1.403 5.55 1.455l3.086-8.982 3.118 8.982c.432-.052 3.401-.414 5.52-1.455z\"/></svg>" } }
