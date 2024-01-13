use leptos::*;
use std::rc::Rc;
use std::cell::{RefCell};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{File, FileReader, HtmlInputElement, DragEvent, ProgressEvent};

const PAGE_SIZE: usize = 16 * 1024; // 16KB pages (like polkavm page size).

#[component]
pub fn FileUploadComponent<F: Fn(Option<Vec<u8>>, bool, String) + 'static>(on_file_uploaded: F) -> impl IntoView {
    let on_file_uploaded = Rc::new(on_file_uploaded);

    // Handler for file upload via input element
    let on_upload = {
        let on_file_uploaded_cloned = on_file_uploaded.clone();
        move |event: web_sys::Event| {
            let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            let files = input.files().unwrap();
            if let Some(file) = files.get(0) {
                let file_rc = Rc::new(file);
                process_file(file_rc, on_file_uploaded_cloned.clone());
            }
        }
    };

    // Handler for file drop
    let on_drop = {
        let on_file_uploaded_cloned = on_file_uploaded.clone();
        move |event: DragEvent| {
            event.prevent_default();
            event.stop_propagation();
            if let Some(data_transfer) = event.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    if let Some(file) = files.get(0) {
                        let file_rc = Rc::new(file);
                        process_file(file_rc, on_file_uploaded_cloned.clone());
                    }
                }
            }
        }
    };

    view! {
        <div
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

fn process_file(file: Rc<File>, on_file_uploaded: Rc<dyn Fn(Option<Vec<u8>>, bool, String)>) {
    let total_size = file.size() as usize;
    let mut current_position = 0;
    let filename = file.name();
    let reader = Rc::new(RefCell::new(FileReader::new().unwrap()));

    let reader_clone = reader.clone();
    let file_clone = file.clone(); // Clone the file Rc

    // Handler for FileReader's onload event
    let onload = Closure::wrap(Box::new(move |_| {
        let mut reader_ref = reader_clone.borrow_mut();
        if let Ok(array_buffer) = reader_ref.result() {
            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
            let page = uint8_array.to_vec();
            let is_last_page = current_position >= total_size;

            // Call the user-provided callback with page data, completion flag, and filename
            on_file_uploaded(Some(page), is_last_page, filename.clone());

            if !is_last_page {
                current_position += PAGE_SIZE;
                read_next_page(&file_clone, current_position, &mut *reader_ref);
            }
        }
    }) as Box<dyn FnMut(ProgressEvent)>);

    reader.borrow().set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
    read_next_page(&file, 0, &mut *reader.borrow_mut());
}


fn read_next_page(file: &Rc<File>, start: usize, reader: &mut FileReader) {
    let end = std::cmp::min(file.size() as usize, start + PAGE_SIZE);
    let blob = file.slice_with_i32_and_i32(start as i32, end as i32).unwrap();
    reader.read_as_array_buffer(&blob).unwrap();
}
