use crate::SimpleIcon; pub struct IconMediatek; impl Default for IconMediatek { fn default() -> Self { IconMediatek } } impl SimpleIcon for IconMediatek { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>MediaTek</title><path d=\"M3.863 8.996c-.296 0-.664.21-.815.467L.064 14.537c-.15.257-.029.467.267.467h19.805c.297 0 .665-.21.816-.467l2.983-5.074c.15-.257.03-.467-.268-.467zm3.41 1.975h1.09l-.232.402h-.776c-.238 0-.312.093-.312.338v.807h1.37l-.233.402H6.502v-1.197c0-.589.307-.752.771-.752zm1.444 0h.937c.703 0 1.002.27 1.002.959 0 .73-.301.99-.976.99h-.963zm7.832 0h1.09l-.233.402h-.775c-.239 0-.313.093-.313.338-.004.264-.002.539-.002.807h1.372l-.233.402h-1.678v-1.197c0-.589.308-.752.772-.752zm-11.567.004v.986l.569-.984.65-.002v1.941h-.547v-1.191l-.672 1.191h-.546v-1.191l-.688 1.19h-.535l1.121-1.938zm5.98 0h.546v1.941h-.545zm1.798 0h.781v1.941h-.553v-1.383l-.797 1.383h-.552zm1.256 0h1.714l-.232.404h-.504v1.537h-.533v-1.537h-.68zm3.873 0h.547v1.941h-.547zm1.345 0h.545l-.558.968-.002.004h.002l.558.969h-.545l-.56-.97zm-9.994.398v1.145h.297c.432 0 .567-.104.567-.586 0-.483-.135-.559-.567-.559zm-1.847.416h.87l-.185.318h-.86zm9.255 0h.872l-.186.318h-.86Z\"/></svg>" } }
