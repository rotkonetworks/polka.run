use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};
use polkavm::ProgramBlob;
use std::rc::Rc;

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{DragEvent, File, FileReader, HtmlInputElement, ProgressEvent};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Navigation/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
                <Route path="disassembler" view=  move || view! { <Disassembler/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <div class="bg-#552BBF flex justify-between">
            <div>
                <a href="/" class="inline-block px-4 py-2 text-#D3FF33 hover:bg-#421E9D  hover:text-#E1FF66">Home</a>
                <a href="/disassembler" class="inline-block px-4 py-2 text-#D3FF33 hover:bg-#421E9D  hover:text-#E1FF66">Disassembler</a>
            </div>
            <div class="hidden md:block">
                <a href="https://github.com/rotkonetworks/polka.run" target="_blank" class="text-#151513 hover:text-#000000">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 250 250" height="40"  width="40">
                        <path fill="#E1FF66" class="octo-background" d="M 249.57771,250.29409 C 163.6834,162.06673 87.406263,88.122635 -0.4222853,0.29408743 H 249.57771 Z"></path>
                        <path fill="currentColor" class="octo-body" d="m 194.57892,71.296301 c -2,-4 -5.00268,-7.999511 -9.00269,-11.999515 -3.99999,-3.999997 -7.9995,-7.002681 -11.9995,-9.002681 -4.00001,-14.000002 -8.99659,-16.998292 -8.99659,-16.998292 -8,3.999999 -11.00464,8.998533 -11.00462,10.998545 -6.00001,0 -10.99732,2.000735 -15.99732,7.000731 -16,16.000004 -10.00195,29.997316 -2.00195,40.997318 -3,0 -6.99854,0.998782 -10.99855,4.998771 L 113.57917,109.2968 c -2,1 -5.99975,-1.00097 -5.99975,-1.00097 l 26.99584,26.99584 c 0,0 -1.99999,-3.99877 0,-4.99877 l 14.00147,-14.00147 c 2.00001,-3 3.00294,-5.9956 3.00293,-7.9956 11,8 23.99732,14.99804 40.99732,-2.00195 5,-5.00001 7.00073,-9.997326 7.00073,-15.997325 -0.90398,-9.744341 -2.80609,-14.23012 -4.99878,-19.000243 z"></path>
                        <path fill="currentColor" class="octo-arm" d="m 121.28633,101.84712 c -14.99999,-9.000009 -8.99999,-19.00001 -8.99999,-19.00001 2.99999,-6.999997 1.99999,-10.999997 1.99999,-10.999997 -1,-7.000002 3,-1.999998 3,-1.999998 4.00001,4.999996 2,11 2,11 -2.99999,9.999998 5.00001,14.999998 9,15.999996" style="transform-origin: 110px 120px;"></path>
                        <path fill="currentColor" class="octo-ear" d="m 210.61601,77.354548 c 0,0 -2.99732,-5.000738 -15.99732,-7.000737 -0.0144,-0.02843 0.007,0.01428 0,0 -0.007,-0.01428 -3.99055,0.468874 -3.99055,0.468874 l 5.4469,19.797551 3.57294,-1.284473 c 2.01561,-1.004006 6.98378,-3.016688 10.96801,-11.981199 z" style="transform-origin: 100px 80px;"></path>
                        <path fill="#E1FF66" class="octo-face" d="m 157.89355,66.610672 c -3.6953,-3.732717 -7.91112,-5.499913 -12.18983,-5.109056 -4.40252,0.402191 -7.13979,2.856546 -7.87463,3.598819 -7.07601,7.147691 -5.83073,16.6566 3.50711,26.774603 1.74973,1.898424 3.65469,3.889989 5.66306,5.91869 1.98876,2.008926 3.9938,3.970152 5.96079,5.829492 11.66194,11.03287 19.88339,7.51635 24.72642,2.62392 4.69996,-4.74761 6.92511,-12.647837 -0.93566,-20.588227 -0.19312,-0.195097 -0.5156,-0.479793 -1.31843,-1.179779 -1.71474,-1.495896 -4.90485,-4.280358 -7.8452,-7.250494 -3.2474,-3.28029 -6.32518,-6.797215 -8.16252,-8.899526 -0.85763,-0.980686 -1.27935,-1.464133 -1.5311,-1.718439 z"></path>
                        <path fill="currentColor" class="octo-mouth" d="m 152.14888,95.779316 c -0.16786,-0.167854 -0.4399,-0.167678 -0.60757,-6e-6 -0.66926,0.669255 -2.20394,0.630882 -3.25661,-0.42178 -0.15188,-0.151888 -0.28137,-0.314719 -0.39094,-0.484545 -0.27436,-0.424649 -0.42501,-0.891266 -0.49172,-1.329545 -0.0135,-0.08772 -0.0233,-0.174308 -0.0303,-0.259131 -0.021,-0.254464 -0.0144,-0.493507 0.0111,-0.702245 0.0423,-0.347542 0.13665,-0.610438 0.24371,-0.717497 0.16767,-0.167672 0.16785,-0.439717 0,-0.607563 -0.16785,-0.167854 -0.43989,-0.167672 -0.60756,-10e-7 -0.0866,0.08662 -0.16552,0.201023 -0.23441,0.337652 -0.10292,0.205165 -0.18327,0.460877 -0.23241,0.749235 -0.0491,0.288359 -0.0669,0.609536 -0.0456,0.945785 0.0215,0.336058 0.0827,0.68736 0.19117,1.036157 0.13593,0.435767 0.34628,0.867413 0.6472,1.260503 0.10041,0.130907 0.21071,0.257691 0.33174,0.37874 0.63088,0.63088 1.38245,0.973575 2.10837,1.079738 0.10362,0.01503 0.20695,0.02546 0.30898,0.03102 0.20444,0.01112 0.40511,0.0038 0.59878,-0.02134 0.48383,-0.06276 0.92265,-0.235099 1.26338,-0.498709 0.0681,-0.05273 0.13235,-0.109022 0.19225,-0.168918 0.16821,-0.167853 0.16803,-0.439896 3.6e-4,-0.607565 z"></path>
                        <path fill="currentColor" class="octo-nose" d="m 150.73021,91.109058 c -0.0362,0.186498 -0.0362,0.378735 -1e-5,0.565239 0.0136,0.06995 0.0323,0.139166 0.0559,0.206948 0.0473,0.135574 0.11495,0.265587 0.20318,0.385376 0.0441,0.05986 0.0933,0.117281 0.14741,0.171438 0.21663,0.21663 0.48382,0.352203 0.76376,0.406177 0.11656,0.0226 0.23528,0.0312 0.35364,0.02547 0.16462,-0.0079 0.32512,-0.05326 0.48042,-0.116025 0.17878,-0.07228 0.34969,-0.17072 0.49477,-0.315797 0.57779,-0.577797 0.57797,-1.514792 1.7e-4,-2.092591 -0.28889,-0.288894 -0.66745,-0.433252 -1.04638,-0.433072 -0.0165,-2.1e-5 -0.0328,0.0041 -0.0495,0.0047 -0.11132,0.0038 -0.22129,0.02224 -0.3296,0.05093 -0.15191,0.04033 -0.29984,0.09899 -0.43505,0.188298 -0.0821,0.05414 -0.15976,0.117478 -0.23203,0.189747 -0.0723,0.07227 -0.13539,0.150099 -0.18975,0.232033 -0.10864,0.163937 -0.18076,0.34467 -0.21699,0.53117 z"></path>
                        <path fill="currentColor" class="octo-left-eye" d="m 153.95489,72.39001 c 0.0493,-0.477547 0.0511,-0.942374 0.004,-1.387104 -0.047,-0.444739 -0.14239,-0.869748 -0.28818,-1.266958 -0.1458,-0.397217 -0.34162,-0.767704 -0.5891,-1.103409 -0.12374,-0.167853 -0.26038,-0.327093 -0.41012,-0.476837 -2.39512,-2.395112 -7.23017,-1.443235 -10.79953,2.126122 -3.56935,3.569358 -4.11631,7.999491 -1.72119,10.394606 0.14974,0.149737 0.30737,0.288005 0.47235,0.414605 0.32979,0.253392 0.68845,0.460702 1.07059,0.621912 0.38197,0.161042 0.78725,0.276167 1.21047,0.34467 0.42286,0.0685 0.86365,0.09074 1.31645,0.06653 0.45263,-0.02439 0.91728,-0.09522 1.38836,-0.212683 1.64857,-0.411202 3.37497,-1.394461 4.93656,-2.956058 1.56159,-1.561594 2.62232,-3.365455 3.12085,-5.101364 0.14275,-0.496019 0.23905,-0.986486 0.28836,-1.464038 z"></path>
                        <path fill="currentColor" class="octo-eye" d="m 176.11077,91.594139 c -2.39512,-2.395112 -7.23018,-1.44324 -10.79954,2.126118 -3.56935,3.569358 -4.11648,7.999323 -1.72119,10.394613 2.39529,2.3953 6.82524,1.84816 10.3946,-1.72119 3.56937,-3.569367 4.52142,-8.404245 2.12613,-10.799541 z" style="transform-origin: 100px 80px;"></path>
                    </svg>
                </a>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
    <div class="p-8">
        <h1 class="text-2xl">"polka.run"</h1>
        <div class="flex flex-wrap gap-6 mt-6">
            <div class="w-1/2">
                <p class="text-md">"Building all things for web3 ecosystem"</p>
            </div>
        </div>
    </div>
    }
}

#[component]
fn Disassembler() -> impl IntoView {
    fn unified_representation(data: &[u8]) -> Vec<String> {
        data.chunks(16)
            .map(|chunk| {
                let hex_part = chunk
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<String>>()
                    .join(" ");
                let text_part: String = chunk
                    .iter()
                    .map(|&byte| {
                        if (32..=126).contains(&byte) || byte == 10 || byte == 13 {
                            byte as char
                        } else {
                            '.'
                        }
                    })
                    .collect();
                format!("{:<48} {}", hex_part, text_part)
            })
            .collect()
    }

    fn disassemble_into(data: &[u8]) -> Result<String, &'static str> {
        let blob = ProgramBlob::parse(data);
        if blob.is_err() {
            return Err("Failed to parse blob");
        }
        let blob = blob.unwrap();

        let mut result = String::new();
        for (nth_instruction, maybe_instruction) in blob.instructions().enumerate() {
            match maybe_instruction {
                Ok(instruction) => {
                    result.push_str(&format!("{}: {}\n", nth_instruction, instruction));
                }
                Err(error) => {
                    result.push_str(&format!(
                        "ERROR: failed to parse raw instruction from blob. nth: {} Error: {}\n",
                        nth_instruction, error
                    ));
                }
            }
        }
        Ok(result)
    }

    let (unified_data, set_unified_data) = create_signal(Vec::new());
    let (disassembled_data, set_disassembled_data) = create_signal(String::new());

    view! {
        <div class="my-0 mx-auto max-w-3xl p-6">
            <h2 class="text-4xl text-center">"polkavm disassembler"</h2>
            <FileUploadComponent on_file_uploaded=move |data_option| {
                if let Some(data) = data_option {
                    set_unified_data(unified_representation(&data));
                    match disassemble_into(&data) {
                        Ok(disassembled) => set_disassembled_data(disassembled),
                        Err(error) => set_disassembled_data(error.to_string())
                    }
                }
            }/>
            //if unified_data().len() > 0 {
                <div class="grid grid-cols-10 gap-6 mt-6">
                    <div class="col-span-7">
                        <h3 class="mb-4 text-2xl">"Uploaded file data:"</h3>
                        <pre class="border border-gray-200 rounded p-2 bg-gray-100 overflow-x-scroll">
                            {
                                move || unified_data().iter().map(|line| view! {
                                    <div class="py-1 font-mono text-xs">{ line.clone() }</div>
                                }).collect::<Vec<_>>()
                            }
                        </pre>
                    </div>
                    <div class="col-span-3">
                        <h3 class="mb-4 text-2xl">"Parsed Instructions:"</h3>
                        <pre class="border border-gray-200 rounded p-2 bg-gray-100 overflow-x-scroll">
                            { move || disassembled_data().clone() }
                        </pre>
                    </div>
                </div>
            //}
        </div>
    }
}

#[component]
fn FileUploadComponent<F: Fn(Option<Vec<u8>>) + 'static>(on_file_uploaded: F) -> impl IntoView {
    let on_file_uploaded = Rc::new(on_file_uploaded);

    let process_file = |on_file_uploaded: Rc<F>, file: File| {
        let reader = FileReader::new().unwrap();
        let reader_c = reader.clone();

        let on_file_uploaded_cloned = on_file_uploaded.clone();
        let onload = Closure::wrap(Box::new(move |_: ProgressEvent| {
            let array_buffer = reader_c
                .result()
                .unwrap()
                .dyn_into::<js_sys::ArrayBuffer>()
                .unwrap();
            let array = js_sys::Uint8Array::new(&array_buffer);
            let vec = array.to_vec();
            on_file_uploaded_cloned(Some(vec));
        }) as Box<dyn FnMut(ProgressEvent)>);

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
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
            class="border-dashed border-4 p-6 mt-6"
            on:drop=on_drop
            on:dragover=move |event: DragEvent| {
                event.prevent_default();
                event.stop_propagation();
            }
        >
            "Drag and drop your .polkavm file here or click to upload"
            <input type="file" accept=".polkavm" on:change=on_upload />
            </div>
    }
}
