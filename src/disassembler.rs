use leptos::*;
use polkavm::ProgramBlob;

use crate::file_upload::FileUploadComponent;

#[component]
pub fn Disassembler() -> impl IntoView {
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

    let version = "0.3"; // TODO: we should generate this from Cargo.toml instead of hardcoding
    let title = format!("polkavm-v{} disassembler", version).to_string();

    view! {
        <div class="flex flex-col container mx-auto">
        <div class="p-4 shadow-md">
        <h2 class="text-4xl text-center">{title}</h2>
        <div class="text-center text-gray-500 text-sm">
        <FileUploadComponent on_file_uploaded=move |data_option| {
        if let Some(data) = data_option {
        set_unified_data(unified_representation(&data));
        match disassemble_into(&data) {
        Ok(disassembled) => set_disassembled_data(disassembled),
        Err(error) => set_disassembled_data(error.to_string())
        }
        }
        }/>
        </div>
        <Show when=move || !unified_data().is_empty()>
        <div class="flex flex-1 overflow-hidden">
        <div class="w-7/10 overflow-auto">
        <h3 class="mb-4 text-2xl">"Binary data:"</h3>
        <pre class="border border-gray-200 rounded p-2 bg-gray-100 overflow-x-scroll">
        {
        move || unified_data().iter().map(|line| view! {
        <div class="py-1 font-mono text-xs">{ line.clone() }</div>
        }).collect::<Vec<_>>()
        }
        </pre>
        </div>
        <div class="w-3/10 overflow-auto">
        <h3 class="mb-4 text-2xl">"Parsed Instructions:"</h3>
        <pre class="border border-gray-200 rounded p-2 bg-gray-100 font-mono text-xs overflow-x-scroll">
        { move || disassembled_data().clone() }
        </pre>
        </div>
        </div>
        </Show>
        </div>
        </div>
    }
}
