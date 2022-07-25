use crate::SimpleIcon; pub struct IconYolo; impl Default for IconYolo { fn default() -> Self { IconYolo } } impl SimpleIcon for IconYolo { fn icon(&self) -> &'static str { "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><title>YOLO</title><path d=\"M0 8.25774c.03921-.20672.14055-.39655.29048-.54416.13641-.1504.32923-.23731.53226-.23989.55612-.00782 1.11225-.01534 1.66838-.02254.42453-.00582.84906-.01186 1.27358-.01811.28696-.00422.57393-.01334.86088-.01256.46616.00346.84127.38416.83781.85032-.00104.13976-.03676.27706-.10396.3996-.08469.15076-.18708.29323-.25689.45035-.13913.30029-.13931.64654-.00049.94698.00954.02185.02019.0432.0319.06397.00398.00698.01316.01102.02482.02024.01383-.01457.02629-.03039.03723-.04725.2515-.46846.4705-.95367.65543-1.45218.09567-.26576.05458-.48797-.13196-.68433-.18066-.19016-.37751-.36491-.55981-.5536-.12492-.12324-.23921-.25681-.34163-.3993-.1515-.2068-.14122-.49056.02485-.68586.22799-.2994.53041-.47341.92244-.457.24789.01037.49598.01587.74397.02399.72891.02386 1.45781.04783 2.18672.0719.38538.01261.77091.02152 1.15611.03814.29836.01287.76704.33392.75135.77064-.00379.11586-.01355.23144-.02926.34629-.00737.05259.00399.0743.05455.07534.06578.00135.13161.00233.19739.00146.82297-.01086 1.53115.28033 2.14192.81948.06985.07376.1695.11169.27072.10304.0793-.00804.16008-.00156.2479-.00156-.00043-.02343-.0035-.04674-.00916-.06948-.13022-.31361-.31488-.57959-.62099-.75203-.09537-.06055-.18088-.13537-.25354-.22186-.0846-.0924-.1617-.19139-.23057-.29604-.13637-.18533-.13812-.43733-.00435-.62454.21142-.34117.59591-.53541.99598-.50316.3074.01637.61551.01953.92331.0282.65735.01853 1.3147.03698 1.97205.05536.37943.01093.75884.02283 1.13822.03569.14936.00077.29857.00952.447.02623.34535.04948.60987.40418.62058.75709.01579.19389-.01119.38889-.07902.57121-.08105.19044-.24483.3333-.44451.38774-.2179.05613-.42139.15788-.59703.29852-.09434.08289-.17548.18074-.26258.27185l.0186.02899c.03399.00207.06799.00587.10198.00593.26623.0005.53249-.00217.79866.00181.07148.00297.14097-.02391.19185-.07421.42159-.39984.94752-.67251 1.51725-.78663 1.02765-.19714 1.94728.05442 2.74953.71624.7651.63118 1.21381 1.45689 1.42431 2.41633.04649.21189.06325.43025.09463.6455.00424.0291.01393.05742.02109.0861v.6095c-.01976.14738-.03553.29541-.05991.44203-.20286 1.2203-.74957 2.23512-1.79181 2.94335-.57291.39155-1.25821.58485-1.95128.55039-.11913-.00449-.23803-.02112-.3564-.03692-.05873-.00784-.0764.01699-.07447.06867.01424.38172.0261.76355.04353 1.14513.01639.35883-.12944.75779-.52908.92134-.17743.07487-.36769.11469-.56025.11726-.61857.00947-1.23738.02194-1.85529.05072-.65944.03071-1.31808.07953-1.97683.12392-.35761.02409-.7144.06747-1.07224.07721-.13999-.00464-.2773-.03961-.40245-.10249-.194-.08189-.36225-.21471-.48692-.38442-.15661-.20164-.16368-.48176-.01743-.69103.40224-.62539.73365-1.28481.89008-2.02022.06798-.31956.03979-.64244.0465-.96422.00128-.06146.00018-.12297.00018-.18446l-.02385-.00761c-.02041.03027-.04209.05977-.06104.09093-.43496.71529-1.01379 1.27475-1.80521 1.57259-1.20058.45182-2.30289.23611-3.28078-.57508-.77831-.64563-1.21358-1.50031-1.41762-2.48015-.01093-.05251-.02081-.10524-.03218-.15765-.00566-.01586-.01256-.03124-.02065-.04601-.02114.03804-.03797.0646-.05127.09281-.12989.27557-.26145.55041-.38684.82801-.02605.05956-.03968.12381-.04005.18882-.00311.52873.00189 1.05752-.00352 1.58621-.00359.20604.08451.40304.24047.53773.16484.14883.35709.26415.56601.3395.36332.13858.55331.55902.55172.85109-.00165.30247-.3426.72613-.66491.7972-1.38286.3049-2.76498.61313-4.14637.9247-.44622.10008-1.03915-.32836-.99918-.82467.02847-.35349.15392-.64559.4984-.83313.59015-.32129.91425-.85064 1.05871-1.49475.10073-.44916-.00481-.87683-.15455-1.29786-.18701-.52584-.4532-1.01336-.72488-1.4984-.40876-.72976-.81441-1.46127-1.22513-2.18992-.1421-.25215-.35782-.44049-.58495-.6118-.20988-.16345-.42934-.31423-.65721-.45151-.26352-.15317-.39205-.3827-.45617-.66437C.01831 8.43963.00837 8.4116 0 8.38323v-.12549zm7.41942 2.79803c.00883.16168.02062.30024.02315.43898.00493.3646.05396.72726.14601 1.08009.23752.89624.68648 1.65899 1.44024 2.21926.58698.45417 1.32735.66271 2.06508.58169.63275-.06873 1.22715-.33727 1.69695-.76668.50117-.4495.87644-1.02193 1.08878-1.66078.07375-.21017.15156-.41892.22751-.62832l.01686.00307v.05913c-.00008.71701.00118 1.43403-.00156 2.15103.00009.11936-.00986.23852-.02975.35621-.14112.80847-.5066 1.52402-.93247 2.2129-.09984.16149-.11127.24748.00851.39723.19724.25268.51166.38465.83016.34845.19673-.01366.39304-.03361.58982-.04655.59049-.03883 1.18074-.08366 1.77178-.1112.77926-.03632 1.5591-.06153 2.33889-.08416.15961-.00342.31726-.03595.46518-.096.26256-.10514.36825-.3496.36219-.60215-.0096-.39989-.02925-.79954-.04325-1.19934-.00397-.11342-.00056-.22712-.00525-.3405-.00332-.08036.02919-.10356.103-.07908.01698.00563.03468.00906.05206.01346.49763.13036 1.02146.12193 1.51464-.02436.81537-.24065 1.43186-.74741 1.88064-1.45445.65266-1.02826.82865-2.1505.5868-3.33639-.18397-.90208-.60394-1.67916-1.31726-2.27567-.73671-.61607-1.58278-.86041-2.53503-.6735-.55446.11568-1.06258.39207-1.46096.79469-.04599.04737-.11.07288-.17596.0701-.19442-.00254-.38889-.00072-.58335-.00074-.27822-.00004-.55643-.00023-.83465-.00056-.05052-.00012-.1047.00118-.06875-.07827.10487-.25187.2602-.4796.45643-.66916.2163-.19047.47312-.3292.75099-.40568.12413-.03002.22676-.11695.2768-.23445.06835-.17034.08406-.35725.0451-.53661-.03235-.22494-.21795-.3964-.44474-.41087-.62162-.01926-1.24326-.03764-1.86494-.05514-.83086-.02437-1.66215-.03952-2.4923-.07883-.21572-.00616-.42694.06235-.59799.19393-.07757.06024-.13981.13797-.18163.22683-.03057.06123-.0349.13227-.012.19676.09109.20588.23885.38161.42603.5067.18281.10836.34405.24954.4756.41644.17987.25426.31305.53851.3933.83945.02257.07415-.01492.08942-.07711.08928-.23633-.00052-.47271-.00245-.70897.00146-.07755.00356-.15283-.02671-.20633-.08297-.70535-.68158-1.55306-.93302-2.51814-.81936-.12828.01511-.25497.04362-.38618.06659.00234-.01864.00131-.02543.0041-.02977.00815-.01248.01707-.02446.02671-.03584.10664-.12747.22299-.24371.28251-.40874.16042-.44483-.12992-.78887-.52121-.81068-.30141-.01681-.60347-.02207-.90525-.03209-.54385-.01806-1.0877-.0359-1.63155-.05351-.5021-.01557-1.00419-.03352-1.50642-.04192-.28441-.00477-.50201.13575-.66391.35987-.08287.10247-.08341.24875-.00131.35184.0877.11635.18265.22705.28429.33144.20198.20708.4199.3993.61399.61326.23665.23957.30304.59935.16746.9076-.01964.04994-.03281.10238-.05118.15287-.23065.63389-.54836 1.22637-.86764 1.81811-.00964.01786-.01718.03684-.0257.0553l-.0333-.01225c-.08141-.14165-.16875-.28026-.24317-.42548-.28027-.54692-.23017-1.07722.09102-1.5909.0475-.07597.09966-.14933.14274-.22769.09995-.17453.07437-.39384-.06306-.54069-.14077-.17484-.35806-.26989-.582-.25461-.6514.02231-1.30341.02763-1.95521.03731-.55621.00827-1.11246.01319-1.6687.01964-.2549.00296-.53734.23476-.56332.48964-.03787.23984.06802.47967.27077.61327.27385.18699.55045.37082.81474.57067.23677.16963.43569.38663.58414.63722.39795.7086.7946 1.41791 1.18997 2.12796.27699.49569.55226.99217.74995 1.52678.16633.44982.28918.90815.21049 1.39237-.13052.80311-.52558 1.43642-1.24915 1.83383-.22743.12492-.31831.38782-.32153.61885-.00408.29269.43478.56272.67574.4978.37767-.10176.7632-.17436 1.14529-.25983.98589-.22051 1.97171-.44131 2.95747-.66239.2524-.05677.47953-.325.46235-.56898-.00779-.23603-.14938-.447-.36487-.54362-.20545-.09575-.40213-.20931-.58777-.33936-.26451-.18692-.41525-.49597-.3997-.81949.00799-.53764.00009-1.0755.0044-1.61322.0008-.0799.0181-.15878.05082-.23168.28786-.62386.58081-1.24538.87246-1.86749.0122-.02607.0265-.05117.05061-.09736zm11.5715 3.53732c0 .09029-.00242.17639.00039.26231.02201.6744.0456 1.34876.06726 2.02317.0051.1588-.01788.17863-.17891.18211-.45432.0098-.90904.01156-1.36281.0332-.49843.02376-.99599.06591-1.49389.10047-.44722.03104-.89442.06246-1.34159.09428-.28018.02032-.56023.04241-.84014.06627-.06956.006-.1466.00932-.17063-.06064-.01511-.05827-.00589-.12017.02553-.17151.41566-.6371.71078-1.34513.87067-2.08885.0422-.18943.06339-.38293.06318-.577.00492-.95297.00403-1.90596-.00266-2.85898-.00325-.51354-.01962-1.027-.0311-1.54047-.0032-.14298-.01286-.15025-.15108-.11607-.24848.0558-.48616.1519-.7036.28448-.06904.03205-.1016.11198-.07461.18315.21874.87979.14547 1.80686-.20877 2.64136-.23307.57463-.62474 1.07113-1.12931 1.43159-.96228.69788-2.28064.63168-3.16816-.1591-.6523-.56228-1.02355-1.28502-1.17915-2.12001-.17228-.86273-.06142-1.75802.31609-2.55267.31601-.6674.77794-1.21048 1.45658-1.52643.97975-.45614 1.89642-.30726 2.73918.35093.10091.07881.18675.17664.28516.25899.03698.03277.08346.0529.13266.05745.4846.00415.96924.00266 1.45388.00286.05638.00002.10437-.00656.08824-.0837-.11261-.5386-.27183-1.05893-.59394-1.5156-.15862-.22974-.36653-.42117-.60855-.56033-.06194-.03523-.13027-.08145-.11028-.15493.01954-.07186.09833-.08391.17235-.08154.72615.02325 1.45233.04569 2.17853.06734.52007.01526 1.04022.02784 1.56032.04206.19427.00531.38849.01526.58277.01629.07483-.00933.14421.04055.1592.11445.01337.07542-.04527.12224-.11813.14566-.33888.10327-.65086.27991-.91376.51736-.41365.38136-.63729.86529-.75875 1.40464-.01507.06689.01115.08783.07146.08767.03589-.0001.07178.00105.10768.00106.68507.00004 1.37015-.00096 2.05521.00117.08049.00313.1582-.02973.21202-.08966.39288-.42163.91283-.70326 1.48061-.80199.77286-.12755 1.4603.07828 2.05969.56953.68984.56538 1.07301 1.31365 1.22738 2.17879.16625.85478.05459 1.74032-.31865 2.52707-.33894.71084-.83905 1.27287-1.57799 1.58337-.74632.3136-1.48212.26309-2.19567-.11774-.03422-.01826-.0687-.03603-.10344-.0533-.00454-.00224-.01148.00033-.03047.00144zm-2.3306.69022v.00053c.1077 0 .21571-.00537.32304.001.3336.01978.60484-.11183.84144-.334.21842-.21742.40765-.46231.56294-.72852.04552-.05502.03782-.13652-.0172-.18204-.00131-.00109-.00265-.00214-.004-.00318-.23576-.25517-.42955-.54614-.57413-.86203-.39392-.84771-.4726-1.73125-.29269-2.64222.02345-.06964.00199-.14653-.05412-.19396-.1091-.0965-.22711-.18244-.35244-.25666-.34992-.17988-.7349-.22175-1.1211-.25029-.06606-.00488-.08271.02927-.08081.0869.01451.43864.02618.87738.0426 1.31595.02839.75779.05833 1.51552.08982 2.2732.0228.55781.04936 1.11547.07016 1.67335.00301.08064.03643.10346.10879.10255.15254-.00192.30513-.00058.4577-.00058zm-7.66719-4.96883c-.0184.18067.05354.35476.13973.523.0854.16668.18933.17002.28754.00779.06417-.10423.11928-.21378.16471-.32744.19212-.51049.49088-.97416.87634-1.36009.0628-.06398.11908-.13405.16804-.20915.11931-.17744.07067-.28749-.13904-.33307-.24648-.0548-.50455.00391-.70306.15995-.51152.37757-.80376.87307-.79426 1.53901zm9.35205-.11107c-.00171.21388.05622.424.16729.6068.07083.09929.14997.10518.22454.00864.04465-.05971.08315-.12378.11491-.19124.05756-.11792.11165-.23751.16226-.35879.15609-.38895.38494-.74456.67427-1.04777.1021-.10477.19672-.21657.28316-.33459.13564-.18525.083-.30163-.14029-.3474-.25638-.05228-.5226.01522-.7231.18333-.49054.37943-.77715.86633-.76304 1.48102zm-.25159-1.11901c-.03346-.00222-.07067-.00677-.10788-.00683-.36151-.00055-.72303-.00066-1.08454-.00034-.28981.00004-.57963.00145-.86942-.00066-.06579-.00048-.10065.02137-.10556.08926-.00666.09213-.01523.18412-.02572.27596-.00868.07646.02319.10522.09966.11121.17538.01374.35154.02723.5248.05623.38135.06383.74383.17765 1.04241.43997.00939.00725.02006.01266.03145.01597.16545-.32795.32823-.65059.4948-.98077zm-4.99681-.00089.47007.90633c.00336.0013.0071.00119.01037-.00032.2888-.17023.60426-.29047.9331-.35564.02862-.01452.04792-.04256.05127-.07448-.00453-.1219-.02249-.2433-.02754-.36521-.00385-.0927-.0449-.11952-.13487-.11862-.40038.00398-.80082.00159-1.20124.00198-.03219.00003-.06437.0037-.10116.00596zm-7.85383 2.45786c.01654-.01838.03178-.03789.04561-.05838.41558-.7264.82629-1.45587 1.13618-2.23512.08967-.23704.16331-.47985.22042-.72677.06308-.25368-.01232-.49126-.13273-.71484-.1802-.33458-.44497-.59741-.72765-.84432-.05048-.03723-.09797-.07836-.14202-.12302-.03163-.0381-.07453-.10873-.05966-.13412.0335-.04692.08471-.07813.14177-.0864.09243-.00476.18509-.00261.2772.00645.49015.01475.98034.02837 1.47047.04382.74408.02347 1.48812.04784 2.23214.07313.07754.00267.17441.0087.19023.09932.01429.08181-.06653.12216-.13516.15806-.54169.28341-.96234.7034-1.32846 1.18323-.45995.6028-.80278 1.27068-1.10486 1.96331-.40241.92268-.81988 1.83882-1.2258 2.75999-.04126.09496-.0634.19712-.06518.30064-.00629.53767.00281 1.07554-.00453 1.61319-.0079.578.23429 1.02072.70694 1.34016.14846.09494.30267.18056.46175.25637.07152.0363.12689.08681.11808.16126-.00841.07101-.07475.10638-.14847.12284-1.3214.29502-2.64257.59105-3.96351.88812-.07599.01708-.1511.00666-.17319-.06685-.01943-.07228.01574-.14809.08348-.17993.8117-.4663 1.27612-1.17724 1.45467-2.08296.09103-.46178.02769-.9151-.10814-1.36114-.18999-.58272-.44205-1.14333-.75184-1.67218-.44746-.80069-.89282-1.60256-1.34242-2.40205-.19272-.34271-.47766-.60425-.78682-.83743-.23356-.17616-.4807-.33444-.7229-.49905-.1063-.07223-.12327-.17973-.03098-.2481.02985-.0186.0645-.02805.09966-.0272.32901-.00434.65807-.00423.98708-.00885.88506-.01245 1.77008-.02784 2.65518-.03661.05379.00864.10242.03705.13635.07967.01435.01588-.0066.0786-.02756.10897-.2005.28045-.33619.60196-.39725.94126-.05598.33167-.01962.67235.10507.98474.17937.46478.47355.8549.80418 1.22061.01637.01489.03399.02833.05267.04018zm11.13476-1.16387c.21416.04999.42003.08311.60529.18777.04931.02479.07287.08209.05525.13439-.15674.93805-.0145 1.90165.40661 2.7544.11921.22382.25153.44042.39627.64865.0416.04412.0433.11248.00395.15861-.12874.1947-.28524.36954-.46453.519-.10183.08527-.23174.12963-.36446.12444-.12823-.00694-.25715-.00293-.38573-.00114-.06372.00089-.09081-.02483-.09324-.08991-.01684-.45039-.03598-.9007-.05448-1.35104-.02928-.71275-.05845-1.42551-.08751-2.13827-.01088-.27742-.01695-.55503-.02428-.83258-.0009-.03432.00396-.06881.00686-.11432zm.3293 1.61169c0 .05331-.00168.09943.00028.14539.01083.25332.02361.50656.0337.7599.01509.37859.02763.75728.04302 1.13586.00702.17278.01986.34531.02675.51809.00235.05896.03135.072.08418.07807.15072.02399.30313-.03165.40294-.14709.06181-.06886.12214-.13917.18648-.20559.04914-.04114.05562-.11432.01447-.16346-.00234-.00279-.0048-.00547-.00739-.00803-.26056-.37651-.45658-.7938-.58-1.23473-.07926-.28583-.1354-.57807-.20443-.87841zm-.03044-.74416.01589.00799c.01414-.14607.02989-.29203.04048-.43835.00138-.01914-.02374-.04019-.0366-.06035-.01282.02162-.03701.04345-.03661.06481.00268.14202.01062.28394.01684.4259z\"/></svg>" } }