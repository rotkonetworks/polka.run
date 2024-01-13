use leptos::*;
use polkavm::ProgramBlob;
use crate::file_upload::FileUploadComponent;
use serde::{Deserialize, Serialize};
// use ron::de::from_str;

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
    // let (toggle_submenu, set_toggle_submenu) = create_signal(false);

    // let item_type = item.item_type.clone();
    // let toggle_submenu_handler = move || {
    //     match item_type {
    //         MenuItemType::SubMenu(_) => {
    //             set_toggle_submenu(!toggle_submenu.get());
    //         }
    //         _ => {}
    //     }
    // };

    view! {
        <div
            role="menuitem"
            class="menu-button px-4 py-2 text-md font-semibold text-gray-700 bg-white hover:bg-gray-100 focus:bg-gray-200 rounded-xs border border-gray-300 shadow-sm cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
        >
            // onclick=toggle_submenu_handler
            {&item.label}
        // <Show when=move || match &item.item_type {
        // MenuItemType::SubMenu(_) => true,
        // _ => false,
        // }>
        // 
        // <div
        // role="menu"
        // class="menu"
        // tabindex="-1"
        // aria-orientation="vertical"
        // aria-labelledby="radix-:R1mqrnnnlaH1:"
        // style="outline:none"
        // >
        // <ul>
        // <For
        // each=move || match &item.item_type {
        // MenuItemType::SubMenu(items) => items.clone().into_iter(),
        // _ => vec![].into_iter(),
        // }
        // key=|item| item.label.clone()
        // children=move |item| {
        // view! { <li><MenuButton item=item.clone() /></li> }
        // }
        // />
        // </ul>
        // </div>
        // </Show>
        </div>
    }
}

// MainMenu
#[component]
fn MainMenu() -> impl IntoView {
    // fn load_menu() -> Result<MainMenu, ron::Error> {
    //     let content = include_str!("pages/disassembler.ron");
    //     println!("{}", content);
    //     from_str(content).map_err(|e| e.into())
    // }
    //
    // let menu = load_menu().expect("Failed to load menu");

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

    let (disassembled_data, set_disassembled_data) = create_signal(String::new());


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

    view! {
        <div class="flex flex-col h-min-screen">
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
                                            set_disassembled_data(String::new());
                                        }
                                    >

                                        <div class="i-mdi-remove" />remove
                                    </li>
                                </ul>
                            </Show>
                        </nav>
                    </aside>
                </Show>
                <div class="flex flex-1 overflow-auto">
                    <div class="w-full h-full">
                        <div class="h-50vh flex flex-row p-4">
                            <Show when=move || unified_data().is_empty()>
                                <div class="border-dashed border-4 w-full h-full p-4">

                                <FileUploadComponent on_file_uploaded=move |data_page, is_last_page, filename| {
                                    static mut ACCUMULATED_DATA: Vec<u8> = Vec::new();

                                    if let Some(page) = data_page {
                                        unsafe {
                                            ACCUMULATED_DATA.extend(page);

                                            if is_last_page {
                                                // Finalize processing once all pages are received
                                                let complete_data = ACCUMULATED_DATA.clone();
                                                set_filename(filename);
                                                set_chunk_size(16);
                                                set_unified_data(
                                                    unified_representation(&complete_data, chunk_size.get() as usize),
                                                );
                                                match disassemble_into(&complete_data) {
                                                    Ok(disassembled) => set_disassembled_data(disassembled),
                                                    Err(error) => set_disassembled_data(error.to_string()),
                                                }

                                                // Clear the accumulated data for future uploads
                                                ACCUMULATED_DATA.clear();
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
                                                        {line.clone()}
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
                        <div class="w-full h-35vh mt-4 border-t border-gray-200 dark:border-gray-800 overflow-x-auto">
                            <div class="text-sm p-4 flex flex-row">
                                <div class="hidden md:block md:w-10/100 flex">
                                    <div class="h-4">
                                        <h3>Offset</h3>
                                    </div>
                                </div>
                                <div class="hidden md:block md:w-23/100 flex">
                                    <div class="h-4">
                                        <h3>HEX</h3>
                                    </div>
                                </div>
                                <div class="w-full md:w-33/100">
                                    <div class="h-4">
                                        <h3>Assembly</h3>
                                    </div>
                                    <Show when=move || !unified_data().is_empty()>
                                        <pre class="border border-gray-200 rounded p-2 bg-gray-100 font-mono text-xs md:text-md xl:text-lg overflow-x-scroll">
                                            {move || disassembled_data.get()}
                                        </pre>
                                    </Show>
                                </div>
                                <div class="hidden md:block md:w-33/100">
                                    <div class="h-4">
                                        <h3>Hint</h3>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
