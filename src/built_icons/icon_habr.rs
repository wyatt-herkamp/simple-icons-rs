use crate::SimpleIcon; pub struct IconHabr; impl Default for IconHabr { fn default() -> Self { IconHabr } } impl SimpleIcon for IconHabr { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>Habr</title><path d=\"M1.14 9.295l.17.253c-.34-.01-.696-.043-.764-.189 0-.007.002-.014.004-.021.19-.05.353-.053.59-.043zm1.806.77l.082.25-.192.008.046-.256c.022 0 .042-.001.064-.003zM3.6 11.89c-.17.156-.301.26-.402.497-.14-.115-.213-.343-.258-.598-.05-.278-.066-.587-.104-.818l.012-.118.197-.014.502.95.053.1zm-.254-2.342a60.527 60.527 0 01.077-.512l.065-.004c-.048.172-.096.344-.142.516zm-.137-2.322l.9.244a1.975 1.975 0 00-.004.04c-.419.31-.806.446-1.014 1.006L3.17 7.27l.039-.043zm.84-1.29c-.058.58.024.836.125 1.196.16.114.224.183.172.352-.27-.134-.556-.274-.97-.252a2.799 2.799 0 01-.03-.035c.116-.457.342-.738.517-1.1l.187-.162zm1.294-.772l.039-.003c-.124.428-.174 1.415-.391 1.807-.513-.243-.503-1.296.071-1.712l.28-.092zm-.478 3.17l.432.469c-.17.522-.491.915-.627 1.52l-.255-.02c-.218-.609.327-1.601.42-2.045l.4.046c.05.125.098.25.147.376l-.517-.347zM6.047 5.16c-.044.507.068.77.098 1.162-.254.13-.509.258-.763.387a5.262 5.262 0 01.373-1.527l.292-.022zm.296-.77l.057-.004-.086.247-.169.011.13-.153.068-.101zM6.4 6.755l-.764.47c.016-.284.378-.667.681-.773l.07.038.013.265zm.51-1.778c-.047.262-.093.523-.138.785l-.143.09-.076.083c-.138-.194-.173-.67-.143-1.032l.5.074zm.508-.59c0 .01-.002.018-.003.026L7.22 4.6c-.13.05-.146.048-.31.042.021-.085.045-.17.068-.255h.44zm.51-1.032L7.7 3.87l-.79-.057.003-.042c.325-.15.61-.328 1.014-.417zm-.412 1.34l.084-.05.073.258-.255-.05.098-.159zm.666.763c-.303.118-.722.085-.965.22a77.912 77.912 0 00-.053-.31l.158-.207c.257.1.514.202.77.303l.09-.006zm-.651.464l-.346.013c-.008-.036-.014-.072-.021-.109.243-.099.651-.108 1.018-.149l-.387.213-.264.032zm.905 0c-.291.073-.57.33-.761.53-.037-.234.412-.646.627-.775.095.085.105.076.134.245zm.51-3.083l-.004.04-.306.185-.454.033.004-.04.76-.218zm-.412.774l-.098-.145c.23-.193.737-.087 1.019-.058l-.921.203zm2.278-.76c.425.035.814.135 1.11.32L12 3.168l-.047.066-.407.12c-.245-.034-.49-.07-.734-.105L9.2 3.02l.004-.033a5.375 5.375 0 011.608-.133zm1.693-.136l-.09.122a1.23 1.23 0 01-.67-.258l.764.097-.004.039zm1.356.954l.211-.08.219.188c-.03.215-.237.391-.396.6l-.122.008c-.332-.55-.751-1.151-1.486-1.363l-.032-.065.297-.12c.499.174.915.438 1.378.59.06.145.041.096-.07.242zm.614 1.223l-.058-.057-.126.009.21-.2-.026.248zm.225 2.848c.344.298.432.73.289 1.29-.148-.332-.295-.665-.444-.997.05-.074.097-.148.146-.222.003-.024.005-.048.009-.071zm.1-2.32l-.004.048-.181.207-.07-.137c.109-.195.03-.09.173-.11l.082-.008zm.212-.003l.806.299c-.001.027-.004.054-.007.08-.068.047-.136.092-.204.137l-.807-.298.007-.081.205-.137zm1.57 1.033l-.447-.297-.062-.12c.194-.12.166-.114.371-.074.045.164.092.327.138.49zm.254-.098l-.09.098c-.108-.056-.127-.127-.164-.259l.233.102.021.059zm-.19-.739l-.064.005c.07-.067.14-.134.212-.2-.05.065-.1.13-.148.195zm1.209 1.298l-.125.055a230.14 230.14 0 01-.681-.383 98.74 98.74 0 01-.213-.29c.353-.26.366.042.699.207l.32.411zm.254.228l-.254-.093.153-.08.101.173zm-.31.416c.092.185.092.133.31.105L17.718 8l-.373-.712.005-.044c.275-.059.363.04.632.08l.103.166-.285.067zm.056.463l.509-.278c-.102.23-.235.348-.392.516l-.117-.238zm1.018.985c-.138.119-.228.157-.489.179l.21.023-.004.037-.717.304-.018-.92.494-.585.09-.043.434 1.005zm-.255-1.412c1.406-.392 1.134.354.507.923-.17-.307-.339-.615-.507-.923zm1.273.149c-.518 2.434-.618 2.567-2.45 3.613-.03-.049-.063-.097-.096-.145l.065-.272c1.195-.34 1.874-.966 1.803-2.383l.599-.77.079-.043zm.062 0c-.02-.084-.04-.168-.062-.252l.19-.006-.128.258zm-3.117 2.367l.018-.269.016-.23.176-.059c.009 0 .017-.002.025-.003.05.305 0 .53-.1.775-.086-.055-.103-.078-.135-.214zm-1.018 2.258l.004-.045.334-.799.07-.168.588.04c.009.044.015.087.018.128.026.304-.079.547-.187.864l-.827-.02zm.255 1.036l.007-.4.01-.1.43.032.062-.005c-.099.163-.197.326-.294.49l-.215-.017zm-.252-1.67l.026-.013.203-.105.023-.002-.03.107-.119.409-.106-.364.003-.032zm-.397 1.285l-.115-.115c.112.078.039.002.115.115zm-.624-.307c.143-.09.298-.062.51-.06l-.063.207c-.175.07-.2.044-.397.023l-.05-.17zm.312.966l.197.466c-.152.103-.305.205-.457.309-.136-.187.012-.71.26-.775zm-.503-1.62l-.064.005.084-.191-.02.187zm.733.072l-.033-.021.112-.236.037-.001.106.248-.222.01zm1.763-3.355l-.014-.087c.067-.082.11-.133.225-.17l.03.152c-.05.033-.1.065-.149.099l-.092.006zm-.014.63c-.046.061-.128.095-.186.144l-.069-1.29.255 1.146zm-.467 1.73c.168-.094.192-.007.212.22h-.509l.009-.08.288-.14zm-.25-.713l.115-.098.093.166-.043.092-.212-.025.047-.135zm-.047.684l-.265.25-.173-.029-.07.007c.04-.079.082-.158.123-.236l.385.008zm-.96-.147c-.106-.074-.03-.064-.05-.12l.05.12zm.802-1.152l-.86-.446c.04-.59.112-1.268.054-1.876.663.24 1.258 1.319.806 2.322zm-.605 0c.211.112.2.214.493.304.051.191-.027.267-.149.47l-.312-.012c-.064-.34.006-.535-.032-.762zm.058 1.669c-.107-.074-.03-.064-.05-.12l.05.12zm1.723-2.484l.003-.048.23-.169c.04.264.025.704-.035 1.032-.136-.106-.161-.554-.198-.815zm0 .815c.052-.086.104-.172.154-.258l-.09.254-.064.004zm1.485-.771l.043-.003c.012.989-.598 1.169-1.4 1.54l-.128.009.5-1.036.985-.51zM17.32 7.103c.008.026.017.051.024.077l-.063.046-.191-.258.23.135zm-.23.43l.007-.045a3.35 3.35 0 01.056-.004l.192.258c-.148-.07-.172-.088-.255-.21zm.254.66l-.343.323-.737-1.07-.956-.83.21-.164c.463.2 1.273.581 1.436 1.057.053.155-.13.283-.006.412.148.154.296.044.397.271zm-2.148 1.613l.078.031.034.744-.119-.028c-.112-.083-.19-.467-.088-.634l.095-.113zm-.564 1.294l.082-.003.085.202-.13.05-.125.006c.03-.085.06-.17.088-.255zm-.232 4.126l-.11-.377c.157-.045.313-.09.47-.137l.04-.002a.527.527 0 00-.005.035l-.395.48zm-.365-1.032c.047-.152.08-.175.165-.259l.07.028.02.066-.157.154-.098.01zm.12.258l-.12-.132.255-.126-.066.253-.069.005zm-.268 1.17c-.036-.028-.07-.057-.106-.087l.078-.005.393-.304.038.31-.301.206-.102-.12zm.02 2.956c-.613.026-.98-.13-1.48-.174l-.573-1.133-.11-.114c.962-.114 1.567-.578 2.28-.899.252.24.823 1.462.32 2.003-.12.185-.255.204-.438.317zm-2.632-1.273l.23-.015c.347.233.583.65.749 1.033l-1.019-.295c.076-.258.108-.52.04-.723zm-.732-.974L11.691 16l1.804.176.032.091c-.537.242-1.148.446-1.764.758-.667.053-.828-.196-1.29-.36l.071-.334zm-.071 1.216l.023-.224.439-.033c.036.195.074.304.02.516l-.482-.26zm-.51.404l.171-.094c.21-.139.367.046.593.119l-.09.088-.673-.113zm.495-.403a15.96 15.96 0 01-.166-.075l-.074-.183.255.016-.015.242zm-.749 0l.255.219c0 .135-.012.12-.043.297l-.212-.516zm-.66-.967c.239-.073.557-.057.915-.065l-.333.258-.685-.067.103-.126zM8.436 3.648c.473-.452 1.659-.317 1.956-.03l.08-.007c-.395.369-1.007.411-1.383.776-.295-.196-.539-.322-.653-.739zm2.6.739l.2.227c-.178.01-.365.021-.552.037-.421.036-.837.099-1.128.25l-.038.002a51.163 51.163 0 01-.063-.099 3.644 3.644 0 011.544-.414l.037-.003zm.992-1.29c.329.33.82.526.99 1.032-.69-.321-1.752-.316-2.29-.827.474.101.878.063 1.3-.205zm.227 1.806l.944.13.074-.006-.757.65c-.12-.239-.148-.563-.261-.774zm1.213 1.104l-.195-.03.254-.042-.06.072zm-.195-.617l.233-.229.021.258-.169-.037-.085.008zm.462 8.54l-.112.005-.096-.09.074-.097.18-.07-.046.253zm-.203.472c.03-.036.062-.072.094-.108l.008-.1.148.235-.255.023.005-.05zm-.101.824l-.158-.19.509-.068-.243.25-.108.008zm.096.774l-.509-.061.36-.197.15.258zm-.357-.728l.103.098-.369.094-.395.02.55-.258.111.046zM11.746 4.129c.297.008.611.083.763.258-.205-.02-.574.013-.667-.145l-.096-.113zm.258 1.032c.145.127.196.277.17.516-.224-.077-.166-.322-.17-.516zm.25 1.627l.009-.044c.507-.11.616.066.385.224-.2-.028-.29-.103-.393-.18zm.653 6.631l.111.028a1.49 1.49 0 00-.004.036l-.25.194.104-.255.039-.003zm-.14-6.294l.19-.157.061.258-.254-.056.002-.045zm.76 6.9l-.615.161-.148.007c0-.008.002-.017.004-.025.23-.046.378-.122.514-.233l.216.04.03.05zm-.439-.606l-.07-.143.175-.115.08.043-.1.208-.085.007zm-1.597 1.785l.303-.236.938.08.032.07-.634.366c-.235-.039-.438-.143-.56-.286l-.08.006zm-.471.022l.47.258-.763-.135c.098-.04.195-.082.293-.123zm-.038.527a7.12 7.12 0 00-.003.062l-.145.08-.107.014.004-.062c.111-.147.092-.098.25-.094zM9.782 5.25a6.022 6.022 0 00-.073-.077c.253-.138.652-.204 1.044-.235.323-.027.641-.03.869-.035l.124.774c-.333-.036-.65-.08-.993-.095a5.2 5.2 0 00-.888.032 41.055 41.055 0 00-.083-.364zm.374 10.355l.062.11-.06.231-.703.054c.075-.23.187-.341.308-.516l.393.121zm-.868-.895c-.32-.018-.758-.252-.852-.516l1.019.259-.167.257zm.358-.014l-.191.014.006-.062.212-.196c-.01.081-.018.162-.027.244zm-.7-9.437l.31-.098.199.25-.21.008-.3-.16zm-.004 11.08c-.017.056-.032.113-.048.17-.025.001-.05.004-.074.007a92.65 92.65 0 00-.129-.258l.255.026-.004.055zm.141-1.113l.594.12.032.08-.461.574-.557-.082.392-.692zm4.444-8.088l.115-.169.042-.001.32.08.032.046-.357.132a41.52 41.52 0 00-.152-.088zm.689-.834l-.18-.11c.183.067.078-.009.18.11zm-.058 1.18l-.122-.104.255-.154-.133.258zm-.122 1.806l.396.098c.107.042.104.053.113.16-.17-.065-.454-.11-.509-.258zm.04-.703l.054-.07.161.257-.255-.05.04-.137zm-5.157 8.491l.027.068-.343.131-.167.013.004-.035.117-.183c.162-.075.166-.026.362.006zm-.737-12.95c.222.166.28.37.245 1.033-.146-.187-.235-.584-.245-1.032zm.146-.515L8.31 3.8l-.127.009.146-.196zm-.146.258c-.007.062-.012.124-.02.186l-.235-.043.194-.139.06-.004zm0 14.968l.126-1.314.892-.235-.938 1.501-.08.048zm.063-3.532l.137-.081c.125.148.013.361-.105.516-.098-.092-.109-.15-.083-.359l.05-.076zm-.827-.126l.293-.471.47.276c-.16.252-.323.504-.485.756-.146-.13-.236-.321-.278-.56zm0 2.021l.47-.428.221.02.073.12c-.02.126-.038.251-.056.376l-.708-.088zm-.763-.35l.56-.336.426.067.032.059-.449.39c-.224-.023-.373-.083-.49-.185l-.08.005zm-.075-.499l.205-.095.088.019.036.042-.593.197-.17-.06.434-.103zm-.689-.242l.15-.363c.278-.036.344.101.553.188l.06.092-.311.213-.253.017-.199-.147zm-.51 1.059l.371-.654c.246 0 .276.02.377.15.045.076.006.035-.092.052l-.023.225.386.06c-.076.189-.088.164-.202.284-.38.02-.551-.052-.816-.117zm-.428-.912l.647.195.036.077-.415.502-.603-.198.335-.576zM2.46 15.56l.008-.077.361.278.031.074c-.751.58.398 2.47 1.705 1.245l.562.226c-.216.385-.587.537-.978.755-.935.078-2.244-1.449-1.69-2.501zm.872.264l.014.175c-.07-.053-.14-.104-.208-.157l-.047-.101.24.083zm1.286 1.096l-.465.36c-.916.096-1.287-.538-1.317-1.317l.179-.216.078-.006c.461.363.869.917 1.495 1.1l.03.079zm-.887-3.76c-.083-.06-.088-.09-.13-.192l.174-.662.203-.178.131.192-.378.84zm-.13.223l.401-.949.082-.048c.185.326.453.474.534.917l-.604.631a1.223 1.223 0 01-.414-.55zm.763.79l.51-.497.253.517c-.254-.008-.508-.014-.763-.02zm1.272-5.215c-.241.515-.562.865-.754 1.364-.084-.453.422-1.104.585-1.504l.078-.045.091.185zm0-1.42c.379-.117.633-.331.904-.571l.115.344-.322.43-.697-.202zm1.273.027l-.138.176-.116-.02.002-.028.232-.21.02.082zm-.2-1.112l.04-.002.16.337-.06.179c-.233-.122-.224-.267-.14-.514zm-.048 7.481l.235.169-.241-.106a1.03 1.03 0 01.006-.063zm-.004.259l.252.109a.9.9 0 00-.004.03l-.54.377c-.155-.07-.174-.209-.22-.36l.005-.032.507-.124zm-.753-.517l.242.098-.13.075-.125.014.013-.187zm-.013-5.16c.306.086.209.028.222.257-.14-.082-.15-.121-.222-.258zm0 4.609l.021-.223c.119.083.16.132.234.258a61.01 61.01 0 00-.255-.035zm.51-3.223c-.096.123-.092.09-.255.161l.111-.258.143.097zm0-1.038l-.16.167c-.16-.09-.085-.054-.013-.221l.095-.037.077.091zm-.021-.439l-.234-.168c.18.069.138.027.234.168zm-.489 4.735l.255.203c-.13-.003-.2-.031-.255-.203zm.764-.01l-.104.01-.089-.164-.062-.08.138-.014c.039.083.077.166.117.248zm-.14-4.778L6.4 8.258c.112.078.038.002.114.115zm.015 4.25l-.13-.27.055-.222.032-.002.168.516a37.21 37.21 0 00-.125-.022zm-.667 3.003l.028.034L5.8 16a.697.697 0 01-.418-.516l.481.142zM5.127 16l.255.226c-.163-.072-.173-.069-.255-.226zm.44-2.288l.03.223-.215-.258.185.035zm-.252 1.207l-.188-.467c.223.085.397.34.51.516l-.322-.049zm.2-4.594l.063-.002-.186.258c-.03-.064.007-.125.11-.2l.013-.057zm-.133 2.218l-.051.102-.146-.113a116.8 116.8 0 01-.058-.123l.2-.022.055.156zM3.6 14.71c.51.332 1.377.64 1.527 1.29A4.234 4.234 0 013.6 14.71zm1.323-.25l.204.25-.509-.258.305.008zm1.864.766l-.387-.39.677-.642.087.707-.377.325zm.377.431l.067-.168.09-.005.097.258-.254-.085zm.343-.947l-.089-.258.185.034-.096.224zm-.343-8.69l.002-.023c.084-.02.168-.04.252-.062l-.176.258-.078-.173zm-2.567 5.335l-.233-.516c.299.065.25.278.233.516zm-.623.258c-.218-.151-.357-.452-.374-.774.447.105.422.281.51.689l-.136.085zm-.263-1.347c-.325-.307.148-1.535.356-1.818.11-.139.248-.136.43-.19l.121.031c-.306.542-.512 1.28-.585 2.034l-.322-.057zm-.475 4.186l.155.261-.139.325-.08.085c.114.016.14-.082.31-.221.33.535.834 1.047 1.39 1.338l-.303.534-.92-.592c-.105-.195.04-.419-.129-.538-.153-.108-.35.048-.506.001l-.432-.396c.127-.284.249-.562.5-.707l.154-.09zm-.88-2.846a6.65 6.65 0 01.149-.764l.034-.003.133.767.07.4c.2.558.402 1.115.604 1.671-.484-.264-.783-.875-.74-1.697-.116.265-.02.682.018.916-.297-.252-.335-.753-.269-1.29zm-.442.265a.941.941 0 01-.086-.31 2.031 2.031 0 01.004-.435l.044-.092.197-.195-.111.721-.048.31zm17.722 2.58l-.509-.196.494-.061.015.258zm-13.47 4.2l.133-.07-.154.257.02-.186zM2.327 9.807l-.217-.02c-.27-.089-.523-.404-.546-.754.413.037.553.233.757.472.002.1.005.201.006.302zm-.03-.021c.03-.557-.395-.905-.947-.89.094-.782.774-1.182 1.38-1.445l.117-.009c-.101.982-.024 1.828-.391 2.678-.271.626-1.232.614-1.18 1.58l.007.08c.137 1.284 1.533 1.545 1.726 2.674-2.07 1.053.453 5.478 2.09 2.879.294.042.555.175.92.047.17-.094.203-.17.269-.366l.585.303.03.074c-.536.416-.914.987-1.49 1.377l.055-.158-.15-.027-.083.045-.075.35c.329.336 1.326-.879 1.618-1.117-.165.577-.573.874-.8 1.402l.07.072c.43-.308.957-1.18 1.037-1.843l.644.107c-.807 1.091-2.88 3.491-3.97 4.132.166-.521.43-.919.564-1.347l-.078.006a14.401 14.401 0 00-.07-.072c-.265.522-.702.889-.78 1.593a.688.688 0 00.174.179c1.296-.216 3.46-3.473 4.494-4.363-.28 2.81-1.786 3.248-3.092 4.95 1.187-.592 1.907-2.102 2.922-2.94C7.781 20.91 7.502 23.533 6.8 24l.116-.009c.788-.49.834-2.505 1.097-3.535l.112-1.083 1.125-1.58.082-.045c.126.395.696.824 1.313.709l.443-.186 1.112.416c.146 1.6.09 2.497-.923 3.715 1.035-.529 1.415-2.567 1.07-3.649 2.407.279 2.31-.533 1.854-2.286l.423.007c-.078-.117-.289-.03-.45-.12-.073-.047-.059-.038-.104-.108.572-.645 1.95-.004 1.66 1.067.396-.888-.479-1.678-1.322-1.36.36-.427.796-.778 1.024-1.343l.082-.044 1.98 2.08.117-.009.09-.122c-.63-.746-1.259-1.493-1.887-2.24l.202-.092a.41.41 0 01.349-.121 22.552 22.552 0 01-.1-.05c1.011.101 2.303.944 3.358.672.097-.169.102-.26.043-.425 1.124.028 3.146-.348 3.79-.894-1.73.363-3.678.486-5.266.274l-1.407-.165-.079.006.379-.68c1.16-.022 4.202-.051 4.817-.779l-.136-.182-.53-.076c-.046.068-.053.132-.06.197l.42.045c-.912.23-3.452.715-4.281.432-.087-.106-.023-.474-.019-.739a.968.968 0 00-.006-.142c1.192-.356 2.077-1.158 2.44-2.368.207-.693.07-1.603.518-2.035.286.025.344.146.554.228-.111-.154-.27-.24-.469-.31l-.373.257c-.341.132-.698-.213-1.277.056-.47-.402-.735-1.037-1.37-1.28l-.065-.11.77-.749c-.252.211-.509.46-.766.71l-.516-.231-.03-.075c.588-.416 1.11-.57 1.779-.438.031.062.064.123.096.185-.031.053-.059.068-.086.083l-.225-.06c.097.057.198.075.299.093l.085-.083.013-.116c-.146-.13-.191-.224-.478-.234l.137-.202c-.328.226-.994.286-1.32.482l.754-.594.09-.121-1.395.832c-.298-.02-.615-.16-.776-.327a39.46 39.46 0 00-.077.006l1.656-1.12-.027-.113c-.628.366-1.257.732-1.884 1.099l-.602-.148.056-.541 4.317-3.39.077-.006-.061-.149c-.16-.076-.817.457-1.065.616-.956.613-1.801 1.376-2.741 1.93.388-.493 1.098-.984 1.216-1.702l-.19-.024-.296.252.322-.14c-.208.633-1.15 1.763-1.682 2.121l-.062-.149c.346-.582.687-1.365.527-2.112l-.29-.17-.311.023c.199.079.334.098.48.195.093.724-.263 1.214-.455 1.8l-.082.044c-.128-.237-.12-.351-.383-.394l-.023-.152.18-.244.124-.086c.042.08.044.051.057.188a.388.388 0 00.059-.196l-.147-.066-.389.412c-.325-.217-.683-.445-1.14-.53.259-.537 1.046-1.4.64-2.081-.496-.828-2.189.126-2.809.783a1.231 1.231 0 00-.186.244c.04-.032.104-.082.186-.144.612-.456 2.241-1.548 2.594-.675.37.487-.339 1.424-.532 1.805-.66-.067-1.35-.13-2.06-.116a7.422 7.422 0 00-2.201.354c-.36.119-1.896 1.039-1.955 1.027l-.7-.678-1.464-.735-.008.077-.086.083 1.876 1.588-.286.175c-.772-.585-1.304-.94-2.453-1.239l-.194.015c.671.462 1.347.885 2.022 1.309l.135.182c-.482.203-.686.625-1.093.887-.636-.304-1.225-.58-1.996-.813l-.054.158 1.444.93.078-.006-.249.172c-.115.387-.34.836-.528 1.382-.72.586-1.653.551-1.798 1.745-.23.063-.392.083-.602.237l-.016.155c.132.184 1.301.679 1.58.727l.172-.166zm8.54-8.256l.107.02-.143-.018c-.025 0-.05.002-.074.003.113-.21.445-.262.764-.239l-.022.152-.624.031-.008.05zm.446.277l.3-.098c.13-.216.204-.445.138-.654-.309-.061-.487.014-.776.136a49.67 49.67 0 00-.157.257l-.06.1a.57.57 0 00.06.06c.098.08.216.108.433.09.06.102-.01.057-.117.085l.18.024zm7.403 2.323l-.322.06.198-.014.227.098-.344.63c.177-.209.353-.357.428-.674l-.187-.1zm-1.85 10.065c-.08-.143-.167-.237-.254-.259.077.073.166.159.254.259zm-1.018 3.096l-.254.148.195-.018.06-.13zm1.018-3.096c.259.292.453.852.397 1.29.294-.635-.041-1.031-.397-1.29Z\"/></svg>" } }