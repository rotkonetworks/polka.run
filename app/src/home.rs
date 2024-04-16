use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container p-8 mx-auto">
            <h1 class="text-2xl font-bold text-center md:text-4xl lg:text-5xl">"polka.run"</h1>
            <div class="flex flex-col lg:flex-row gap-6 mt-6">
                <div class="w-full lg:w-2/4 p-4">
                    <p class="text-md leading-relaxed mt-4">
                        "PolkaVM is general purpose virtual machine for user-level applications. "
                        "It runs on x86 architechture linux based operating systems and transpiles. "
                        "Rust/C/asm guest programs into RISC-V based bytecode. We have built graphical "
                        "interface for disassembler to improve accessibility to understand binaries PolkaVM produces. "
                    </p>
                </div>
                <div class="w-full lg:w-2/4">
                    <pre class="text-xs xl:text-sm">
                        "╔══════════════════════════════════════════════════════════════════════════════╗\n"
                        "║...:OdKK;  ;OK;..OcXXXd0XxdkdOXNX0oX0lNdNWNWWx0WkWo0OdWXkWdK ; kNMX0d0XMMx;XMk║\n"
                        "║...:OdKKxccx00.;.x:XXXo0XX0XNNXNX0lX0lXdNNNNNx.,.No0OdWXkWx0 ' kWMX0d0XMMx;NMx║\n"
                        "║...c0dXXKKxXXK.' dcXXXoOXK0dc;,... ......',;cc,:cWoKOdWXkNx0 . kWWXo:lKMMk;WMx║\n"
                        "║'''c0dXXKKxKKK.. dcXXx;'.            .      ...;:x:OkoWXkNx0 . kWWX . xMMMkMM0║\n"
                        "║.,,cOoOKKKxK0K.. o:;.                .         ..'.co:Oxd0x0 . OWWX . xMMMkMMk║\n"
                        "║;;;c;  lKKkK0Kkl,';.          .';::;'.         .....:,dc:cco . OWWX.. xMMMMMMx║\n"
                        "║clcc;  oNXOKd' . .,        .ollodxkkKWWo'          .;,c;;:::   OWWX.  xMMMMMMN║\n"
                        "║dxdcxcl0NXo'   .  '   .cooxxc0Nc...,kWWWMO.         ,';'';;:cco0WMNW0NXMMMMMMx║\n"
                        "║O0OlNXKN0;...  .  .     .:Mkoo' OMl  kNNWMN.      ..;::..',;:coOWMWWdNXMMMMMMk║\n"
                        "║00OlNX0o'....  .  .       :0.    .  .OKXNNMM0:.   o;odk'.',,;:lkXWNXlXNMMMMMMW║\n"
                        "║OOOlNKc'...... .  .        .xl    .lkOKXXNWWMOxx,  .',...',,,;:dkKK0;kNMWMMMMK║\n"
                        "║000l0:,''......'  . .   ..   ox  ;cldk0KKNWMo:dcdKd'    ..,;,,;ocdxx;lxNWWNlll║\n"
                        "║000c:;,''''....'  ' .   ok'   X'.;:coxO0KXW,ld'odloOKc.   .'''.,'cll,lokNNN;..║\n"
                        "║KK0';,,,'''''..,  ' .         ;O.,;:cok0KXM.;O:o,dokkXMx'   ...c.xd;:ooo0WNlcc║\n"
                        "║KKk.,''''',,'..,  ' .          kc.,;coxO0XWd oOklxkkOXWKNX:.  ..lO0llONOxWWNNW║\n"
                        "║XKo.;;;;,,,'''.,. ' '     .     od,;codkOKNWk..kXNXXXX0KKWMMk,...'cl'oO0OKMNNX║\n"
                        "║XKc':::;,,,''..;. ' '     .      .dOxxxk0KXNWMd..:0KXX0NWWNkoXWo,.,,':clokOWNK║\n"
                        "║XK,'c::;,,,'...,'',;o,.             'oNWXXXXNWWMNl'.,odXNXdMd. .;ll:.;:cl...,X║\n"
                        "║X0::k0l;,,'..:...,,.'.d:               .MdoOMMMMMMMMk:'';dWMNMk,...,,cccl,;;lW║\n"
                        "║WNllkXKx:,'.c' ,Occk..'X:.              M.  ,MMMMMMMMoc:'..,;oNMKl;,':cloOMMMO║\n"
                        "║MWxlcldxc,'.,c.;:lO';.dK;..       .;:coooc:;,''KMMMMMd      ...;ldNx;clodNMMW;║\n"
                        "║MWKllccc:;,,.;d:.;..:0x;,.    ,;:'   xc        .XMMMMMK,    ...,;clxxKkxOMMMW:║\n"
                        "║WWNxllcc::;;,'.;codl:;;;,.   .o.co.co. .:.      .KMMMMMMO,...',;:ccl:xO0W;'MWd║\n"
                        "║WWXKxllcc:::;;,,',l;;;;,'.    ,llccc:;. .        .OMMMMMMM0c;;:cclll;dxNMMMWMW║\n"
                        "║KK0OKdll;,,::;;,.':;;;;,..   oc.dcoc.xd           .kMMMMMMMMNkollloo;dXMMNWWMK║\n"
                        "║K0Ok0Kxl . koc:,:dd0k:,'...  .','  .'..        .....dMMMMMMWWWM0kddd:XMMMXWNMX║\n"
                        "║NKOkKXK0c::KKxc:cokXNXOc,'.... .. ....,;::,.........;oNWMMMWNNNWWNOOlWWMK0M0W;║\n"
                        "║KOxxKXXXNK0KKkoc::coOXXXkc;,''.::...',;cllc:;;;;:;;dOdoKWWWWWNXXXNMO...,. ;MMl║\n"
                        "║KO:::XXNNNNXOd',''',.,,;k0dc:;;;cdc;;;:::ccccccccc:ldOOdONNWWNNNWMoc...d..,Wxx║\n"
                        "║KO . XNNNNNN0O,o;..; ..:k0MKdlcccdO0dccccclcllloooloodxxxkKNNWWMMMOd,.'x'.:Mc:║\n"
                        "║0k   WWWWNNN00;c........dKMMMKxooooxkkdlolooooddddddddxxO0NMWMWMWMk0.'';','K::║\n"
                        "║X0lclWWWWNNN00;, ; ''''.OKMWMWWKxddddxxdddddddxkxxxkOxXKWWMMkONMWMk0 . . . K;;║\n"
                        "║KKK0XNXNXXXX0Klx c.,,,';0XMMMMMWNKxkxxkkkkkOO0x00NNWMxMXNNMKlxKMNM0K,:;;,:;0ol║\n"
                        "║XNX0Xo..NXXNXNN0KKXXXXXNNWWWMMMMMXxMWkMNMMWMMMkNKMMWMOMNNNMMNMWMNMMM0MMMMMMMMM║\n"
                        "╚══════════════════════════════════════════════════════════════════════════════╝\n"
                    </pre>
                </div>
           </div>
        </div>
    }
}
