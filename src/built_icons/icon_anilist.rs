use crate::SimpleIcon; pub struct IconAnilist; impl Default for IconAnilist { fn default() -> Self { IconAnilist } } impl SimpleIcon for IconAnilist { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>AniList</title><path d=\"M6.361 2.943L0 21.056h4.942l1.077-3.133H11.4l1.052 3.133H22.9c.71 0 1.1-.392 1.1-1.101V17.53c0-.71-.39-1.101-1.1-1.101h-6.483V4.045c0-.71-.392-1.102-1.101-1.102h-2.422c-.71 0-1.101.392-1.101 1.102v1.064l-.758-2.166zm2.324 5.948l1.688 5.018H7.144z\"/></svg>" } }
