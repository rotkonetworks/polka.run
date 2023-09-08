use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use polkavm::ProgramBlob;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::*;

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
        <nav class="bg-#552BBF">
            <a href="/" class="inline-block px-4 py-2 text-#D3FF33 hover:bg-#421E9D  hover:text-#E1FF66">Home</a>
            <a href="/disassembler" class="inline-block px-4 py-2 text-#D3FF33 hover:bg-#421E9D  hover:text-#E1FF66">Disassembler</a>
        </nav>
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

    let process_file = |on_file_uploaded: Rc<F>, file: web_sys::File| {
        let reader = web_sys::FileReader::new().unwrap();
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
