use crate::SimpleIcon; pub struct IconAib; impl Default for IconAib { fn default() -> Self { IconAib } } impl SimpleIcon for IconAib { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>AIB</title><path d=\"M4.8125.004c-.045.0116-.0796.0506-.0977.1073-.1266.4015-.6012 3.0184.7286 3.129 4.032.3342 3.338 3.6082 6.2832 4.2148v1.0703H9.6328l-.9453.9297h3.039v1.8183H9.1642c-2.0105 0-2.8492-1.3788-2.5196-2.4277.2838-.9032 2.2103-2.6043-.1308-3.334.7429 2.0266-2.4297 1.7762-2.4297 4.8945 0 2.4346 2.8034 3.3262 5.08 3.3262h6.3712c2.2766 0 5.08-.8916 5.08-3.3262 0-3.1183-3.1726-2.868-2.4297-4.8945-2.341.7297-.4126 2.4308-.1289 3.334.3296 1.0489-.511 2.4277-2.5214 2.4277h-2.5625V9.4551h3.039l-.9453-.9297h-2.0937V7.459c2.39-.49 2.6793-2.8833 4.3183-4.0488.7411-.5268 2.0625-.5293 2.0625-.5293s-1.8286-1.3563-2.0254-1.502c-.1964-.1456-.6423-.455-1.168-.455-.6355 0-1.5042.9231-2.1015 1.3046-1.279.818-4.0935 2.129-9.086-2.17-.059-.0509-.1151-.0663-.1601-.0546zm11.4766 1.7577c.3063 0 .5546.248.5546.5547a.5545.5545 0 01-.5546.5547.5543.5543 0 01-.5547-.5547c0-.3067.248-.5547.5547-.5547zM5.8945 15.9551L2.713 24H4.375l.7363-1.8867h3.127L8.9843 24h1.6876l-3.0137-7.625c-.0984-.2494-.3452-.42-.6133-.42zm5.17 0v.3672c.0472.0203.0956.043.1386.0625.2022.1428.3457.4904.3457.8984l.002 6.4297c0 .1582.127.2871.2851.2871h1.8243v-.3672c-.0476-.021-.0969-.043-.1407-.0625-.2022-.1428-.3457-.4925-.3457-.9004V16.242c0-.1578-.127-.287-.2851-.287zm3.832 0V24h3.8047c1.6017 0 2.586-1.0737 2.586-2.1816 0-.8782-.5965-1.6329-1.4532-1.9746.579-.3973.9531-1.0128.9531-1.707 0-1.1076-.9842-2.1817-2.586-2.1817zm1.5156 1.3887h1.8496c.534 0 .9668.4332.9668.9667 0 .5179-.4078.9386-.92.963h-1.8964zm-9.7266.8945l1 2.4765H5.6797zm9.7266 2.4433h2.3828c.534 0 .9356.4294.9356.963 0 .5335-.4349.9667-.9688.9667h-2.3496Z\"/></svg>" } }
