use crate::SimpleIcon; pub struct IconKonami; impl Default for IconKonami { fn default() -> Self { IconKonami } } impl SimpleIcon for IconKonami { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Konami</title><path d=\"m14.167 12.562.59-1.298a1.53 1.53 0 0 0 .062-.158h.012c.013.04.037.095.061.158l.575 1.298zm1.887 1.325h1.036l-1.635-3.537a.396.396 0 0 0-.359-.233h-.717c-.041 0-.041.04-.012.055.085.044.146.19.081.325l-1.582 3.39h.702l.39-.87h1.713zm-4.089-3.77v2.152c0 .107.004.174.008.269h-.008a6.068 6.068 0 0 0-.273-.348l-1.618-1.871c-.127-.147-.229-.202-.461-.202H8.79c-.037 0-.041.04-.013.055.123.056.22.123.22.345v3.37h.616v-2.425c0-.13-.004-.23-.008-.34h.008c.114.154.27.356.396.502l1.944 2.263h.322a.305.305 0 0 0 .306-.305v-3.465zm11.733 0h-.856c-.04 0-.045.04-.016.055.126.056.224.123.224.345v3.37H24v-3.465a.3.304 0 0 0-.302-.305m-1.386 3.77-.562-3.442a.401.401 0 0 0-.384-.328h-.53l-.921 2.144a1.866 1.866 0 0 0-.09.23h-.008a1.935 1.935 0 0 0-.081-.218l-.816-1.91a.401.401 0 0 0-.367-.246h-.807c-.04 0-.045.04-.016.055.11.048.192.131.155.34l-.55 3.375h.582l.367-2.382c.017-.118.041-.268.045-.344h.004c.037.1.086.218.139.34l1.015 2.386h.302l1.027-2.429c.057-.142.098-.245.126-.324h.004c.013.095.029.237.053.38l.38 2.373zm-16.205-.25c-.758 0-1.19-.739-1.19-1.59 0-.973.432-1.685 1.19-1.685s1.19.744 1.19 1.59c0 1.001-.432 1.686-1.19 1.686m0-3.66c-1.272 0-2.21.887-2.21 2.022 0 1.14.865 2.022 2.21 2.022 1.272 0 2.206-.883 2.206-2.022 0-1.135-.86-2.021-2.206-2.021M4.33 13.85c-.327-.07-.58-.225-.856-.506-.302-.309-1.387-1.586-1.387-1.586l1.729-1.642h-.934L1.305 11.66c-.07.067-.11.11-.147.154H1.15c.004-.051.004-.107.004-.158v-1.234a.3.304 0 0 0-.302-.305h-.82c-.036 0-.044.04-.012.055.123.056.22.123.22.345v3.37h.914V12.15c0-.047 0-.079-.004-.13h.008c.032.051.09.11.147.182 0 0 .962 1.131 1.064 1.238.407.427.978.578 1.957.483.053-.004.053-.06.004-.072\"/></svg>" } }