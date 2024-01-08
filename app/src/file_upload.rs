use leptos::*;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{File, FileReader, HtmlInputElement, DragEvent, ProgressEvent};

#[component]
pub fn FileUploadComponent<F: Fn(Option<Vec<u8>>) + 'static>(on_file_uploaded: F) -> impl IntoView {
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
        class="border-dashed border-4 p-6 mt-6 w-full h-full"
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
