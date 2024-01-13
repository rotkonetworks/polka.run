mod bin;

use crate::bin::{Binary, CodeLine, LoadError};

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    use leptos_meta::provide_meta_context;
    use leptos_router::{Route, Router, Routes};

    provide_meta_context();

    view! {
        <body class="h-screen w-screen">
            <Navigation/>
            <Router>
                <Routes>
                    <Route path="" view=move || view! { <Home/> }/>
                    <Route path="disassembler" view=move || view! { <Disassembler/> }/>
                </Routes>
            </Router>
        </body>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <div class="bg-#552BBF flex justify-between">
            <div>
                <a
                    href="/"
                    class="inline-block px-4 py-2 text-#D3FF33 hover:bg-#421E9D  hover:text-#E1FF66"
                >
                    Home
                </a>
                <a
                    href="/disassembler"
                    class="inline-block px-4 py-2 text-#D3FF33 hover:bg-#421E9D  hover:text-#E1FF66"
                >
                    Disassembler
                </a>
            </div>
            <div class="hidden md:block">
                <a
                    href="https://github.com/rotkonetworks/polka.run"
                    target="_blank"
                    class="text-#151513 hover:text-#000000"
                >
                    <GithubSVG/>
                </a>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="container p-8 mx-auto">
            <h1 class="text-3xl font-bold text-center md:text-4xl lg:text-5xl">"polka.run"</h1>
            <div class="flex flex-col md:flex-row gap-6 mt-6">
                <div class="lg:w-1/2 p-4">
                    <h1 class="text-3xl font-bold">
                        "PolkaVM: Stepping up the Blockchain Virtual Machines"
                    </h1>
                    <p class="text-md leading-relaxed mt-4">
                        "PolkaVM, a new RISC-V based virtual machine by Polkadot, promises to transform the blockchain landscape. Jan from Polkadot's team at Parity introduced this innovative VM, highlighting its unique features and potential benefits."
                    </p>
                    <h2 class="text-xl mt-4 font-semibold">
                        "Background: Polkadot and WebAssembly"
                    </h2>
                    <p class="text-md leading-relaxed mt-2">
                        "Polkadot has heavily utilized WebAssembly (WASM) since its inception, utilizing it for both its state transition function, known as the runtime, and its native smart contract solution. WASM's speed and efficiency initially made it an attractive choice. However, it presented several challenges."
                    </p>
                    <ul class="list-disc pl-5 mt-2">
                        <li>
                            <strong>"Complexity"</strong>
                            " - WASM's instruction set grew from 172 to over 400 instructions, creating a complex and constantly evolving environment."
                        </li>
                        <li>
                            <strong>Determinism</strong>
                            " - Blockchain technology demands 100% determinism, but WASM does not fully meet this requirement."
                        </li>
                    </ul>
                </div>
                <div class="lg:w-1/2 p-4">
                    <h2 class="text-xl font-semibold">"PolkaVM's Advantages"</h2>
                    <ul class="list-disc pl-5 mt-2">
                        <li>
                            <strong>"Performance"</strong>
                            " - Early benchmarks show PolkaVM nearly matching native performance."
                        </li>
                        <li>
                            <strong>"Efficiency"</strong>
                            " - Exceptional compile-time performance, significantly outpacing competitors."
                        </li>
                        <li>
                            <strong>"Simplicity and Stability"</strong>
                            " - Leverages the RISC-V architecture for a simpler, stable baseline."
                        </li>
                        <li>
                            <strong>"Security"</strong>
                            " - Runs guest programs in separate processes and namespaces, similar to Docker containers."
                        </li>
                    </ul>
                    <h2 class="text-xl mt-4 font-semibold">"Future Prospects"</h2>
                    <p class="text-md leading-relaxed mt-2">
                        "PolkaVM, still in its research phase, has shown impressive results in a short period."
                    </p>
                    <ul class="list-disc pl-5 mt-2">
                        <li>
                            <strong>"Time-Travel Debugging"</strong>
                            " - A unique feature allowing backward navigation during debugging."
                        </li>
                        <li>
                            <strong>"Cross-Platform Compatibility"</strong>
                            " - Aiming to support different CPU architectures."
                        </li>
                        <li>
                            <strong>"Optimization and Extensions"</strong>
                            " - Continuous improvements in performance and support for RISC-V extensions."
                        </li>
                    </ul>
                </div>
                <div class="lg:w-1/2">
                    <pre class="text-xs xl:text-sm">
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

#[component]
fn GithubSVG() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 250 250" height="40" width="40">
            <path
                fill="#E1FF66"
                class="octo-background"
                d="M 249.57771,250.29409 C 163.6834,162.06673 87.406263,88.122635 -0.4222853,0.29408743 H 249.57771 Z"
            ></path>
            <path
                fill="currentColor"
                class="octo-body"
                d="m 194.57892,71.296301 c -2,-4 -5.00268,-7.999511 -9.00269,-11.999515 -3.99999,-3.999997 -7.9995,-7.002681 -11.9995,-9.002681 -4.00001,-14.000002 -8.99659,-16.998292 -8.99659,-16.998292 -8,3.999999 -11.00464,8.998533 -11.00462,10.998545 -6.00001,0 -10.99732,2.000735 -15.99732,7.000731 -16,16.000004 -10.00195,29.997316 -2.00195,40.997318 -3,0 -6.99854,0.998782 -10.99855,4.998771 L 113.57917,109.2968 c -2,1 -5.99975,-1.00097 -5.99975,-1.00097 l 26.99584,26.99584 c 0,0 -1.99999,-3.99877 0,-4.99877 l 14.00147,-14.00147 c 2.00001,-3 3.00294,-5.9956 3.00293,-7.9956 11,8 23.99732,14.99804 40.99732,-2.00195 5,-5.00001 7.00073,-9.997326 7.00073,-15.997325 -0.90398,-9.744341 -2.80609,-14.23012 -4.99878,-19.000243 z"
            ></path>
            <path
                fill="currentColor"
                class="octo-arm"
                sd="m 121.28633,101.84712 c -14.99999,-9.000009 -8.99999,-19.00001 -8.99999,-19.00001 2.99999,-6.999997 1.99999,-10.999997 1.99999,-10.999997 -1,-7.000002 3,-1.999998 3,-1.999998 4.00001,4.999996 2,11 2,11 -2.99999,9.999998 5.00001,14.999998 9,15.999996"
                style="transform-origin: 110px 120px;"
            ></path>
            <path
                fill="currentColor"
                class="octo-ear"
                d="m 210.61601,77.354548 c 0,0 -2.99732,-5.000738 -15.99732,-7.000737 -0.0144,-0.02843 0.007,0.01428 0,0 -0.007,-0.01428 -3.99055,0.468874 -3.99055,0.468874 l 5.4469,19.797551 3.57294,-1.284473 c 2.01561,-1.004006 6.98378,-3.016688 10.96801,-11.981199 z"
                style="transform-origin: 100px 80px;"
            ></path>
            <path
                fill="#E1FF66"
                class="octo-face"
                d="m 157.89355,66.610672 c -3.6953,-3.732717 -7.91112,-5.499913 -12.18983,-5.109056 -4.40252,0.402191 -7.13979,2.856546 -7.87463,3.598819 -7.07601,7.147691 -5.83073,16.6566 3.50711,26.774603 1.74973,1.898424 3.65469,3.889989 5.66306,5.91869 1.98876,2.008926 3.9938,3.970152 5.96079,5.829492 11.66194,11.03287 19.88339,7.51635 24.72642,2.62392 4.69996,-4.74761 6.92511,-12.647837 -0.93566,-20.588227 -0.19312,-0.195097 -0.5156,-0.479793 -1.31843,-1.179779 -1.71474,-1.495896 -4.90485,-4.280358 -7.8452,-7.250494 -3.2474,-3.28029 -6.32518,-6.797215 -8.16252,-8.899526 -0.85763,-0.980686 -1.27935,-1.464133 -1.5311,-1.718439 z"
            ></path>
            <path
                fill="currentColor"
                class="octo-mouth"
                d="m 152.14888,95.779316 c -0.16786,-0.167854 -0.4399,-0.167678 -0.60757,-6e-6 -0.66926,0.669255 -2.20394,0.630882 -3.25661,-0.42178 -0.15188,-0.151888 -0.28137,-0.314719 -0.39094,-0.484545 -0.27436,-0.424649 -0.42501,-0.891266 -0.49172,-1.329545 -0.0135,-0.08772 -0.0233,-0.174308 -0.0303,-0.259131 -0.021,-0.254464 -0.0144,-0.493507 0.0111,-0.702245 0.0423,-0.347542 0.13665,-0.610438 0.24371,-0.717497 0.16767,-0.167672 0.16785,-0.439717 0,-0.607563 -0.16785,-0.167854 -0.43989,-0.167672 -0.60756,-10e-7 -0.0866,0.08662 -0.16552,0.201023 -0.23441,0.337652 -0.10292,0.205165 -0.18327,0.460877 -0.23241,0.749235 -0.0491,0.288359 -0.0669,0.609536 -0.0456,0.945785 0.0215,0.336058 0.0827,0.68736 0.19117,1.036157 0.13593,0.435767 0.34628,0.867413 0.6472,1.260503 0.10041,0.130907 0.21071,0.257691 0.33174,0.37874 0.63088,0.63088 1.38245,0.973575 2.10837,1.079738 0.10362,0.01503 0.20695,0.02546 0.30898,0.03102 0.20444,0.01112 0.40511,0.0038 0.59878,-0.02134 0.48383,-0.06276 0.92265,-0.235099 1.26338,-0.498709 0.0681,-0.05273 0.13235,-0.109022 0.19225,-0.168918 0.16821,-0.167853 0.16803,-0.439896 3.6e-4,-0.607565 z"
            ></path>
            <path
                fill="currentColor"
                class="octo-nose"
                d="m 150.73021,91.109058 c -0.0362,0.186498 -0.0362,0.378735 -1e-5,0.565239 0.0136,0.06995 0.0323,0.139166 0.0559,0.206948 0.0473,0.135574 0.11495,0.265587 0.20318,0.385376 0.0441,0.05986 0.0933,0.117281 0.14741,0.171438 0.21663,0.21663 0.48382,0.352203 0.76376,0.406177 0.11656,0.0226 0.23528,0.0312 0.35364,0.02547 0.16462,-0.0079 0.32512,-0.05326 0.48042,-0.116025 0.17878,-0.07228 0.34969,-0.17072 0.49477,-0.315797 0.57779,-0.577797 0.57797,-1.514792 1.7e-4,-2.092591 -0.28889,-0.288894 -0.66745,-0.433252 -1.04638,-0.433072 -0.0165,-2.1e-5 -0.0328,0.0041 -0.0495,0.0047 -0.11132,0.0038 -0.22129,0.02224 -0.3296,0.05093 -0.15191,0.04033 -0.29984,0.09899 -0.43505,0.188298 -0.0821,0.05414 -0.15976,0.117478 -0.23203,0.189747 -0.0723,0.07227 -0.13539,0.150099 -0.18975,0.232033 -0.10864,0.163937 -0.18076,0.34467 -0.21699,0.53117 z"
            ></path>
            <path
                fill="currentColor"
                class="octo-left-eye"
                d="m 153.95489,72.39001 c 0.0493,-0.477547 0.0511,-0.942374 0.004,-1.387104 -0.047,-0.444739 -0.14239,-0.869748 -0.28818,-1.266958 -0.1458,-0.397217 -0.34162,-0.767704 -0.5891,-1.103409 -0.12374,-0.167853 -0.26038,-0.327093 -0.41012,-0.476837 -2.39512,-2.395112 -7.23017,-1.443235 -10.79953,2.126122 -3.56935,3.569358 -4.11631,7.999491 -1.72119,10.394606 0.14974,0.149737 0.30737,0.288005 0.47235,0.414605 0.32979,0.253392 0.68845,0.460702 1.07059,0.621912 0.38197,0.161042 0.78725,0.276167 1.21047,0.34467 0.42286,0.0685 0.86365,0.09074 1.31645,0.06653 0.45263,-0.02439 0.91728,-0.09522 1.38836,-0.212683 1.64857,-0.411202 3.37497,-1.394461 4.93656,-2.956058 1.56159,-1.561594 2.62232,-3.365455 3.12085,-5.101364 0.14275,-0.496019 0.23905,-0.986486 0.28836,-1.464038 z"
            ></path>
            <path
                fill="currentColor"
                class="octo-eye"
                d="m 176.11077,91.594139 c -2.39512,-2.395112 -7.23018,-1.44324 -10.79954,2.126118 -3.56935,3.569358 -4.11648,7.999323 -1.72119,10.394613 2.39529,2.3953 6.82524,1.84816 10.3946,-1.72119 3.56937,-3.569367 4.52142,-8.404245 2.12613,-10.799541 z"
                style="transform-origin: 100px 80px;"
            ></path>
        </svg>
    }
}

//------------------------------------------------------------------------------
// DISASSEMBLER

#[derive(Clone, Debug)]
enum State {
    Blank,
    Good(Binary),
    Bad(LoadError),
}

#[component]
fn Disassembler() -> impl IntoView {
    use State::*;

    let (state, set_state) = create_signal(Blank);

    let load_binary = move |data_option: Option<Vec<u8>>| {
        if let Some(data) = data_option {
            match bin::load(&data) {
                Ok(bin) => {
                    set_state(Good(bin))
                }
                Err(error) => {
                    set_state(Bad(error))
                }
            };
        }
    };

    view! {
        <main>
            {move || match state() {
                Blank => view! {
                    <FileReceptacle on_file_uploaded={load_binary} />
                },
                Good(bin) => view! {
                    <BinaryView binary={bin} />
                },
                Bad(error) => view! {
                    <Error message={error} />
                },
            }}
        </main>
    }
}

#[component]
fn BinaryView(binary: Binary) -> impl IntoView {
    view! {
        <div>
            <MemoryView memory={binary.memory} />
            <CodeView code={binary.code} />
        </div>
    }
}

#[component]
fn MemoryView(memory: Vec<String>) -> impl IntoView {
    view! {
        <div>
            <pre class="border w-full border-gray-200 rounded p-2 bg-gray-100 overflow-x-scroll">
                {move || {
                    memory.iter()
                        .map(|line| {
                            view! {
                                <div class="py-1 font-mono text-xs md:text-md xl:text-lg">
                                    {line}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </pre>

            <table class="w-full border-collapse table-fixed">
                <thead>
                <tr>
                    <th class="sticky top-0 bg-gray-300 border p-2 text-left w-1/10">
                        Offset
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        00
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        01
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        02
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        03
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        04
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        05
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        06
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        07
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        08
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        09
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0A
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0B
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0C
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0D
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        "0E"
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0F
                    </th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td class="border p-2 bg-gray-200">000000</td>
                    <td class="border p-2">41</td>
                    <td class="border p-2">42</td>
                    <td class="border p-2">4F</td>
                    <td class="border p-2">41</td>
                    <td class="border p-2">42</td>
                    <td class="border p-2">4F</td>
                    <td class="border p-2">41</td>
                    <td class="border p-2">42</td>
                    <td class="border p-2">4F</td>
                    <td class="border p-2">41</td>
                    <td class="border p-2">42</td>
                    <td class="border p-2">4F</td>
                    <td class="border p-2">4F</td>
                    <td class="border p-2">41</td>
                    <td class="border p-2">42</td>
                    <td class="border p-2">4F</td>
                </tr>
                <tr>
                    <td class="border p-2 bg-gray-200">000010</td>
                    <td class="border p-2">11</td>
                    <td class="border p-2">F2</td>
                    <td class="border p-2">FF</td>
                </tr>
                </tbody>
            </table>

            <table class="w-full border-collapse table-fixed">
                <thead>
                <tr>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        00
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        01
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        02
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        03
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        04
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        05
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        06
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        07
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        08
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        09
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0A
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0B
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0C
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0D
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        "0E"
                    </th>
                    <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                        0F
                    </th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td class="border p-2">.</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">F</td>
                    <td class="border p-2">U</td>
                    <td class="border p-2">N</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">c</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">.</td>
                    <td class="border p-2">v</td>
                    <td class="border p-2">i</td>
                    <td class="border p-2">i</td>
                    <td class="border p-2">.</td>
                </tr>
                <tr>
                    <td class="border p-2">11</td>
                    <td class="border p-2">F2</td>
                    <td class="border p-2">FF</td>
                </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn CodeView(code: Vec<CodeLine>) -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th>Offset</th>
                    <th>Hex</th>
                    <th>Assembly</th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    code.iter()
                        .map(|ins| {
                            view! {
                                <tr>
                                    <td>{ins.offset.to_string()}</td>
                                    <td>{ins.hex.to_string()}</td>
                                    <td>{ins.text.to_string()}</td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </tbody>
        </table>
    }
}

// TODO: Refactor.
#[component]
fn FileReceptacle<F: Fn(Option<Vec<u8>>) + 'static>(on_file_uploaded: F) -> impl IntoView {
    use wasm_bindgen::{closure::Closure, JsCast};
    use web_sys::{File, FileReader, HtmlInputElement, DragEvent, ProgressEvent};
    use std::rc::Rc;

    let on_file_uploaded = Rc::new(on_file_uploaded);

    let process_file = |on_file_uploaded: Rc<F>, file: File| {
        let reader = FileReader::new().unwrap();
        let reader_c = reader.clone();

        let on_file_uploaded_cloned = on_file_uploaded.clone();
        let on_load = Closure::wrap(Box::new(move |_: ProgressEvent| {
            let array_buffer = reader_c
                .result()
                .unwrap()
                .dyn_into::<js_sys::ArrayBuffer>()
                .unwrap();
            let array = js_sys::Uint8Array::new(&array_buffer);
            let vec = array.to_vec();
            on_file_uploaded_cloned(Some(vec));
        }) as Box<dyn FnMut(ProgressEvent)>);

        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();
        reader.read_as_array_buffer(&file).unwrap();
    };

    let on_upload = {
        let on_file_uploaded_cloned = on_file_uploaded.clone();
        move |event: web_sys::Event| {
            let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let files = input.files().unwrap();
            if let Some(file) = files.get(0) {
                process_file(on_file_uploaded_cloned.clone(), file);
            }
        }
    };

    let on_drop = {
        let on_file_uploaded_cloned = on_file_uploaded.clone();
        move |event: DragEvent| {
            event.prevent_default();
            event.stop_propagation();
            if let Some(data_transfer) = event.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    if let Some(file) = files.get(0) {
                        process_file(on_file_uploaded_cloned.clone(), file);
                    }
                }
            }
        }
    };

    view! {
        <div
            class="border-dashed border-4 p-6 mt-6 w-full h-full"
            on:drop=on_drop
            on:dragover=move |event: DragEvent| {
                event.prevent_default();
                event.stop_propagation();
            }
        >
            "Drag and drop your .polkavm file here or click to upload"
            <input type="file" accept=".polkavm" on:change=on_upload/>
        </div>
    }
}

#[component]
fn Error(message: String) -> impl IntoView {
    view! {
        <p>{message}</p>
    }
}
