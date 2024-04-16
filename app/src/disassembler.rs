use leptos::*;
use polkavm_common::program::{ProgramBlob, Instruction};
use crate::file_upload::FileUploadComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
struct DisassembledLine {
    offset: String,
    hex: String,
    assembly: String,
    operation: String,
}

// Helper function to create a new DisassembledLine
impl DisassembledLine {
    fn new(offset: usize, hex: String, assembly: String, operation: String) -> Self {
        Self {
            offset: format!("{:06X}", offset),
            hex,
            assembly,
            operation,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum MenuItemType {
    RegularItem,
    SubMenu(Vec<MenuItem>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MenuItem {
    label: String,
    item_type: MenuItemType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MainMenu {
    items: Vec<MenuItem>,
}

#[component]
fn MenuButton(item: MenuItem) -> impl IntoView {

    view! {
        <div
            role="menuitem"
            class="menu-button px-4 py-2 text-md font-semibold text-gray-700 bg-white hover:bg-gray-100 focus:bg-gray-200 rounded-xs border border-gray-300 shadow-sm cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
        >
            {&item.label}
        </div>
    }
}

// MainMenu
#[component]
fn MainMenu() -> impl IntoView {

    let menu = MainMenu {
        items: vec![
            MenuItem {
                label: "File".to_string(),
                item_type: MenuItemType::SubMenu(vec![
                    MenuItem {
                        label: "Load New".to_string(),
                        item_type: MenuItemType::RegularItem,
                    },
                    MenuItem {
                        label: "Unload All".to_string(),
                        item_type: MenuItemType::RegularItem,
                    },
                ]),
            },
            MenuItem {
                label: "Settings".to_string(),
                item_type: MenuItemType::RegularItem,
            },
            MenuItem {
                label: "View".to_string(),
                item_type: MenuItemType::SubMenu(vec![
                    MenuItem {
                        label: "Style".to_string(),
                        item_type: MenuItemType::SubMenu(vec![
                            MenuItem {
                                label: "System Default".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                            MenuItem {
                                label: "Day Mode".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                            MenuItem {
                                label: "Dark Mode".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                        ]),
                    },
                    MenuItem {
                        label: "Zoom".to_string(),
                        item_type: MenuItemType::SubMenu(vec![
                            MenuItem {
                                label: "Zoom: {zoom_level}%".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                            MenuItem {
                                label: "Zoom in (+)".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                            MenuItem {
                                label: "Zoom out (-)".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                            MenuItem {
                                label: "Default Size".to_string(),
                                item_type: MenuItemType::RegularItem,
                            },
                        ]),
                    },
                ]),
            },
            MenuItem {
                label: "Compare".to_string(),
                item_type: MenuItemType::RegularItem,
            },
            MenuItem {
                label: "Info".to_string(),
                item_type: MenuItemType::RegularItem,
            },
        ],
    };

    view! {
        <nav
            role="menubar"
            class="flex h-10 items-center space-x-1 rounded-md border-0 bg-background p-1"
            tabindex="0"
            data-orientation="horizontal"
            style="outline:none"
        >
            <For
                each=move || menu.items.clone().into_iter()
                key=|item| item.label.clone()
                children=move |item| {
                    view! { <MenuButton item=item.clone()/> }
                }
            />

        </nav>
    }
}

// Main component
#[component]
pub fn Disassembler() -> impl IntoView {

    let (unified_data, set_unified_data) = create_signal(Vec::new());
    let (chunk_size, set_chunk_size) = create_signal(0u8);
    let (filename, set_filename) = create_signal(String::new());
    let (show_file_options, set_show_file_options)  = create_signal(false);

    let (disassembled_data, set_disassembled_data) = create_signal(Vec::<DisassembledLine>::new());


    fn unified_representation(data: &[u8], chunk_size: usize) -> Vec<String> {
        let hex_length = chunk_size * 3 - 1; // Expected length of the hex part

        data.chunks(chunk_size)
            .enumerate()
            .map(move |(index, chunk)| {
                let current_offset = index * chunk_size; // Calculate offset here

                // Initialize hex_part and text_part
                let mut hex_part = String::with_capacity(hex_length);
                let mut text_part = String::with_capacity(chunk_size);

                for &byte in chunk {
                    // Append to hex_part and text_part
                    hex_part.push_str(&format!("{:02x} ", byte));
                    text_part.push(if (32..=126).contains(&byte) { byte as char } else { '.' });
                }

                // Pad the hex_part and text_part if necessary
                while hex_part.len() < hex_length {
                    hex_part.push_str("..");
                    text_part.push(' ');
                }

                // Ensure hex_part and text_part are of consistent length
                let hex_part_padded = format!("{:width$}", hex_part, width = hex_length);
                let text_part_padded = format!("{:<width$}", text_part, width = chunk_size);

                // Format the output string with the current offset
                format!("{:06x} {} {}", current_offset, hex_part_padded, text_part_padded)
            })
            .collect()
    }


    fn serialize_instruction(instruction: &Instruction) -> (String, usize) {
        let mut buffer = [0u8; 16]; // maximum instruction size?
        let size = instruction.serialize_into(&mut buffer);

        let hex_representation = buffer[..size].iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ");

        (hex_representation, size)
    }

    fn find_first_instruction_binary(blob: &ProgramBlob) -> Result<Vec<u8>, String> {
        let first_instruction = blob.instructions().next()
            .ok_or("No instructions in the blob")?
            .map_err(|e| format!("Error parsing first instruction: {}", e))?;

        let (binary, _) = serialize_instruction(&first_instruction);

        // Convert the binary string to a byte array
        let mut result = Vec::new();
        for byte_str in binary.split_whitespace() {
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|e| format!("Error parsing byte: {}", e))?;
            result.push(byte);
        }

        Ok(result)
    }

    // Function to find the true byte offset of the first instruction in the blob
    fn find_true_byte_offset(blob: &ProgramBlob, first_instruction_binary: &[u8]) -> Result<usize, String> {
        let blob_bytes = blob.as_bytes();

        blob_bytes.windows(first_instruction_binary.len())
            .position(|window| window == first_instruction_binary)
            .ok_or_else(|| "Binary sequence not found".to_string())
    }

    // Updated calculate_program_base_address function
    fn calculate_program_base_address(blob: &ProgramBlob) -> Result<usize, String> {
        let first_instruction_binary = find_first_instruction_binary(blob)?;
        println!("First instruction binary: {:?}", first_instruction_binary);
        find_true_byte_offset(blob, &first_instruction_binary)

    }

    fn disassemble_into_lines(data: &[u8]) -> Result<Vec<DisassembledLine>, &'static str> {
        let blob = ProgramBlob::parse(data).map_err(|_| "Failed to parse blob")?;

        // Assuming you have a function that calculates the true base address of the program
        let program_base_address = calculate_program_base_address(&blob)
            .map_err(|_| "Failed to calculate program base address")?;

        let mut result = Vec::new();
        let mut current_byte_offset: usize = 0; // Start from the calculated program base address

        for maybe_instruction in blob.instructions() {
            match maybe_instruction {
                Ok(instruction) => {
                    let mut serialized = [0u8; 32]; // Adjust size as needed
                    let size = instruction.serialize_into(&mut serialized);
                    let hex_buffer = serialized[..size]
                        .iter()
                        .map(|byte| format!("{:02X} ", byte))
                        .collect::<String>();

                    // Extract the opcode name from the instruction
                    let opcode_name = format!("{:?}", instruction.opcode());

                    // Calculate the true offset for this instruction
                    let true_offset = current_byte_offset + program_base_address;

                    result.push(DisassembledLine::new(
                        true_offset,
                        hex_buffer,
                        instruction.to_string(),
                        opcode_name, // Pass the opcode name to the operation field
                    ));
                    current_byte_offset += size;
                },
                Err(error) => {
                    // Even in case of an error, calculate the true offset
                    let true_offset = current_byte_offset + program_base_address;

                    result.push(DisassembledLine::new(
                        true_offset,
                        "ERROR".to_string(),
                        format!("Error: {}", error),
                        "Unknown".to_string(), // Use a placeholder like "Unknown" for errors
                    ));
                }
            }
        }
        Ok(result)
    }

    view! {
        <div class="flex flex-col">
            <div class="flex h-16 w-full items-center px-4 md:px-6 bg-gray-100 dark:bg-gray-800">
                <div>
                    <MainMenu/>
                </div>
            </div>
            <div class="flex flex-1 overflow-auto">
                <Show when=move || !unified_data().is_empty()>
                    <aside class="w-32 md:w-40 lg:w-48 xl:w-64 bg-gray-200 dark:bg-gray-700 p-2 lg:p-4 overflow-auto">
                        <nav class="p-2 lg:p-4 bg-gray-100 w-full shadow-md">
                            <a
                                class="flex items-center text-sm hover:bg-gray-200 p-1"
                                href="#"
                                rel="ugc"
                                on:click=move |_| {
                                    set_show_file_options(!show_file_options.get());
                                }
                            >

                                <div class="i-mdi-file" /><span class="truncate">{filename}</span>
                            </a>
                            <Show when=move || show_file_options.get()>
                                <ul class="list-none p-0 m-0 text-xs">
                                    <li
                                        class="cursor-pointer hover:bg-gray-300 p-2 rounded overflow-auto"
                                        on:click=move |_| {
                                            set_unified_data(Vec::new());
                                            set_chunk_size(0);
                                            set_filename(String::new());
                                            set_disassembled_data(Vec::new());
                                        }
                                    >

                                        <a><div class="i-mdi-remove" />remove</a>
                                    </li>
                                </ul>
                            </Show>
                        </nav>
                    </aside>
                </Show>
                <div class="flex flex-1 overflow-auto">
                    <div class="w-full h-full">
                        <div class="h-60vh flex flex-row p-4">
                            <Show when=move || unified_data().is_empty()>
                                <div class="border-dashed border-4 w-full h-full p-4">
                                    <FileUploadComponent on_file_uploaded=move |
                                        data_option,
                                        filename|
                                    {
                                        if let Some(data) = data_option {
                                            set_filename(filename);
                                            set_chunk_size(16);
                                            set_unified_data(
                                                unified_representation(&data, chunk_size.get() as usize),
                                            );

                                            match disassemble_into_lines(&data) {
                                                Ok(disassembled) => set_disassembled_data(disassembled),
                                                Err(error) => {
                                                    set_disassembled_data(Vec::new());
                                                    println!("{}", error);
                                                }
                                            }
                                        }
                                    }/>
                                </div>
                            </Show>
                            <Show when=move || !unified_data().is_empty()>
                                <pre class="border w-full h-full border-gray-200 rounded p-2 bg-gray-100 overflow-x-scroll">
                                    {move || {
                                        unified_data()
                                            .iter()
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
                            </Show>
                        </div>
                        <header class="flex h-16 w-full items-center px-4 md:px-6 bg-gray-100 dark:bg-gray-800">
                            <div
                                role="menubar"
                                class="flex h-10 items-center space-x-1 rounded-md border bg-background p-1"
                                tabindex="0"
                                data-orientation="horizontal"
                                style="outline:none"
                            >
                                <button
                                    type="button"
                                    role="menuitem"
                                    id="radix-:R1mqrnnnlaH1:"
                                    aria-haspopup="menu"
                                    aria-expanded="false"
                                    data-state="closed"
                                    class="flex cursor-default select-none items-center rounded-sm px-3 py-1.5 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground"
                                    tabindex="-1"
                                    data-orientation="horizontal"
                                    data-radix-collection-item=""
                                >
                                    Disassembler
                                </button>
                            </div>
                        </header>

                        <div class="w-full mt-4 border-t border-gray-200 dark:border-gray-800 overflow-x-auto">
                            {/* flex container for headers */}
                            <div class="flex divide-x divide-gray-200">
                                <div class="flex-1 p-2 font-bold text-left bg-gray-200">"Offset"</div>
                                <div class="flex-1 p-2 font-bold text-left bg-gray-200">"Hex"</div>
                                <div class="flex-1 p-2 font-bold text-left bg-gray-200">"Assembly"</div>
                                <div class="flex-1 p-2 font-bold text-left bg-gray-200">"Operation"</div>
                            </div>

                            {/* Flex container for content */}
                            <Show when=move || !disassembled_data().is_empty()>
                                {move || disassembled_data().iter().map(|line| {
                                    view! {
                                        <div class="flex divide-x divide-gray-200">
                                            <div class="flex-1 p-2 bg-white">
                                                <pre class="whitespace-pre-wrap overflow-x-auto">{&line.offset}</pre>
                                            </div>
                                            <div class="flex-1 p-2 bg-white">
                                                <pre class="whitespace-pre-wrap overflow-x-auto">{&line.hex}</pre>
                                            </div>
                                            <div class="flex-1 p-2 bg-white">
                                                <pre class="whitespace-pre-wrap overflow-x-auto">{&line.assembly}</pre>
                                            </div>
                                            <div class="flex-1 p-2 bg-white">
                                                <pre class="whitespace-pre-wrap overflow-x-auto">{&line.operation}</pre>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </Show>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
