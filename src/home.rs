use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container p-8 mx-auto">
        <h1 class="text-3xl font-bold text-center md:text-4xl lg:text-5xl">"polka.run"</h1>
        <div class="flex flex-col md:flex-row gap-6 mt-6">
        <div class="md:w-1/2">
        <p class="text-md leading-relaxed">
        "This site unofficial educational content and resources for PolkaVM like tutorials and
        graphical disassembler."
        </p>
        <p class="text-md leading-relaxed">
        "PolkaVM is a general-purpose, user-level virtual machine based on the RISC-V architecture, 
        designed for a range of applications from embedded systems to large-scale cloud computing."
        </p>
        <h2 class="text-xl mt-4 font-semibold">"Key Features"</h2>
        <ul class="list-disc pl-5">
        <li><strong>"Security"</strong>
        " - Utilizing sandboxing and a secure-by-design approach for enhanced protection."</li>
        <li><strong>"Determinism"</strong>
        " - Ensuring consistent and predictable performance, crucial for applications like smart contracts."</li>
        <li><strong>"Performance"</strong>
        " - Leveraging the efficiency of the RISC-V architecture for optimized execution and high-speed computations."</li>
        </ul>
        </div>
        <div class="md:w-1/2">
        <pre>
"╔══════════════════════════════════════════════════════════════════════════════╗
║...:OdKK;  ;OK;..OcXXXd0XxdkdOXNX0oX0lNdNWNWWx0WkWo0OdWXkWdK ; kNMX0d0XMMx;XMk║
║...:OdKKxccx00.;.x:XXXo0XX0XNNXNX0lX0lXdNNNNNx.,.No0OdWXkWx0 ' kWMX0d0XMMx;NMx║
║...c0dXXKKxXXK.' dcXXXoOXK0dc;,... ......',;cc,:cWoKOdWXkNx0 . kWWXo:lKMMk;WMx║
║'''c0dXXKKxKKK.. dcXXx;'.            .      ...;:x:OkoWXkNx0 . kWWX . xMMMkMM0║
║.,,cOoOKKKxK0K.. o:;.                .         ..'.co:Oxd0x0 . OWWX . xMMMkMMk║
║;;;c;  lKKkK0Kkl,';.          .';::;'.         .....:,dc:cco . OWWX.. xMMMMMMx║
║clcc;  oNXOKd' . .,        .ollodxkkKWWo'          .;,c;;:::   OWWX.  xMMMMMMN║
║dxdcxcl0NXo'   .  '   .cooxxc0Nc...,kWWWMO.         ,';'';;:cco0WMNW0NXMMMMMMx║
║O0OlNXKN0;...  .  .     .:Mkoo' OMl  kNNWMN.      ..;::..',;:coOWMWWdNXMMMMMMk║
║00OlNX0o'....  .  .       :0.    .  .OKXNNMM0:.   o;odk'.',,;:lkXWNXlXNMMMMMMW║
║OOOlNKc'...... .  .        .xl    .lkOKXXNWWMOxx,  .',...',,,;:dkKK0;kNMWMMMMK║
║000l0:,''......'  . .   ..   ox  ;cldk0KKNWMo:dcdKd'    ..,;,,;ocdxx;lxNWWNlll║
║000c:;,''''....'  ' .   ok'   X'.;:coxO0KXW,ld'odloOKc.   .'''.,'cll,lokNNN;..║
║KK0';,,,'''''..,  ' .         ;O.,;:cok0KXM.;O:o,dokkXMx'   ...c.xd;:ooo0WNlcc║
║KKk.,''''',,'..,  ' .          kc.,;coxO0XWd oOklxkkOXWKNX:.  ..lO0llONOxWWNNW║
║XKo.;;;;,,,'''.,. ' '     .     od,;codkOKNWk..kXNXXXX0KKWMMk,...'cl'oO0OKMNNX║
║XKc':::;,,,''..;. ' '     .      .dOxxxk0KXNWMd..:0KXX0NWWNkoXWo,.,,':clokOWNK║
║XK,'c::;,,,'...,'',;o,.             'oNWXXXXNWWMNl'.,odXNXdMd. .;ll:.;:cl...,X║
║X0::k0l;,,'..:...,,.'.d:               .MdoOMMMMMMMMk:'';dWMNMk,...,,cccl,;;lW║
║WNllkXKx:,'.c' ,Occk..'X:.              M.  ,MMMMMMMMoc:'..,;oNMKl;,':cloOMMMO║
║MWxlcldxc,'.,c.;:lO';.dK;..       .;:coooc:;,''KMMMMMd      ...;ldNx;clodNMMW;║
║MWKllccc:;,,.;d:.;..:0x;,.    ,;:'   xc        .XMMMMMK,    ...,;clxxKkxOMMMW:║
║WWNxllcc::;;,'.;codl:;;;,.   .o.co.co. .:.      .KMMMMMMO,...',;:ccl:xO0W;'MWd║
║WWXKxllcc:::;;,,',l;;;;,'.    ,llccc:;. .        .OMMMMMMM0c;;:cclll;dxNMMMWMW║
║KK0OKdll;,,::;;,.':;;;;,..   oc.dcoc.xd           .kMMMMMMMMNkollloo;dXMMNWWMK║
║K0Ok0Kxl . koc:,:dd0k:,'...  .','  .'..        .....dMMMMMMWWWM0kddd:XMMMXWNMX║
║NKOkKXK0c::KKxc:cokXNXOc,'.... .. ....,;::,.........;oNWMMMWNNNWWNOOlWWMK0M0W;║
║KOxxKXXXNK0KKkoc::coOXXXkc;,''.::...',;cllc:;;;;:;;dOdoKWWWWWNXXXNMO...,. ;MMl║
║KO:::XXNNNNXOd',''',.,,;k0dc:;;;cdc;;;:::ccccccccc:ldOOdONNWWNNNWMoc...d..,Wxx║
║KO . XNNNNNN0O,o;..; ..:k0MKdlcccdO0dccccclcllloooloodxxxkKNNWWMMMOd,.'x'.:Mc:║
║0k   WWWWNNN00;c........dKMMMKxooooxkkdlolooooddddddddxxO0NMWMWMWMk0.'';','K::║
║X0lclWWWWNNN00;, ; ''''.OKMWMWWKxddddxxdddddddxkxxxkOxXKWWMMkONMWMk0 . . . K;;║
║KKK0XNXNXXXX0Klx c.,,,';0XMMMMMWNKxkxxkkkkkOO0x00NNWMxMXNNMKlxKMNM0K,:;;,:;0ol║
║XNX0Xo..NXXNXNN0KKXXXXXNNWWWMMMMMXxMWkMNMMWMMMkNKMMWMOMNNNMMNMWMNMMM0MMMMMMMMM║
╚══════════════════════════════════════════════════════════════════════════════╝"
        </pre>
        </div>
        </div>
        <div class="mt-6">
        <p class="text-md">
        "PolkaVM's combination of security, determinism, and performance positions it as a promising 
        lightweight virtual machine, making it ideal for a wide range of applications."
        </p>
        </div>
        </div>
    }
}
