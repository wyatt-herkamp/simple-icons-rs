use crate::SimpleIcon; pub struct IconMicrosoftonenote; impl Default for IconMicrosoftonenote { fn default() -> Self { IconMicrosoftonenote } } impl SimpleIcon for IconMicrosoftonenote { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Microsoft OneNote</title><path d=\"M23 1.5Q23.41 1.5 23.7 1.8 24 2.09 24 2.5V21.5Q24 21.91 23.7 22.2 23.41 22.5 23 22.5H7Q6.59 22.5 6.3 22.2 6 21.91 6 21.5V18H1Q0.59 18 0.3 17.7 0 17.41 0 17V7Q0 6.59 0.3 6.3 0.58 6 1 6H6V2.5Q6 2.09 6.3 1.8 6.59 1.5 7 1.5ZM4.56 11 7.39 15.93H9.18V8.07H7.44V13.1L4.71 8.07H2.82V15.93H4.56ZM22.5 21V18H19.5V21ZM22.5 16.5V13.5H19.5V16.5ZM22.5 12V9H19.5V12ZM22.5 7.5V3H7.5V6H11Q11.41 6 11.7 6.3 12 6.59 12 7V17Q12 17.41 11.7 17.7 11.41 18 11 18H7.5V21H18V7.5Z\"/></svg>" } }
