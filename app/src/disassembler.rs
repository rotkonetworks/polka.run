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
fn FileIcon() -> impl IntoView {
    view! {
    <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewbox="0 0 24 24"
        fill="none"
        stroke="currentcolor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="w-5 h-5"
    >
        <path d="m4 20h16a2 2 0 0 0 2-2v8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2a2 2 0 0 0 7.93 3h4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2z"></path>
    </svg>
    }
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
        <div role="menuitem" class="menu-button">
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
            class="flex h-10 items-center space-x-1 rounded-md border bg-background p-1"
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
        <div class="h-full w-full flex flex-col">
            <header class="flex h-16 w-full items-center px-4 md:px-6 bg-gray-100 dark:bg-gray-800">
                <div>
                    <MainMenu/>
                </div>
            </header>
            <main class="flex flex-1 w-full h-full">
                <aside class="w-9/100 bg-gray-200 dark:bg-gray-700 p-4 overflow-auto">
                    <nav class="space-y-1">
                        <a class="flex items-center space-x-2 text-sm" href="#" rel="ugc">
                            <FileIcon/>
                            <span class="overflow-auto">hello_world.polkavm</span>
                        </a>
                        <ul>
                            <li>export</li>
                            <li>remove</li>
                        </ul>
                        <a class="flex items-center space-x-2 text-sm" href="#" rel="ugc">
                            <FileIcon/>
                            <span>doom.polkavm</span>
                        </a>
                    </nav>
                </aside>
                <div class="flex flex-1 overflow-auto">
                    <div class="w-full h-full p-4">
                        <div class="h-3/5 flex flex-row">
                            <Show when=move || unified_data().is_empty()>
                                <FileUploadComponent on_file_uploaded=move |data_option| {
                                    if let Some(data) = data_option {
                                        set_chunk_size(16);
                                        set_unified_data(unified_representation(&data, chunk_size.get() as usize));
                                        match disassemble_into(&data) {
                                            Ok(disassembled) => set_disassembled_data(disassembled),
                                            Err(error) => set_disassembled_data(error.to_string()),
                                        }
                                    }
                                }/>
                            </Show>
                            <Show when=move || !unified_data().is_empty()>
                                <pre class="border w-full border-gray-200 rounded p-2 bg-gray-100 overflow-x-scroll">

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
                            //     <div class="md:w-54/100">
                            //         <div class="overflow-x-auto m-4">
                            //             <table class="w-full border-collapse table-fixed">
                            //                 <thead>
                            //                     <tr>
                            //                         <th class="sticky top-0 bg-gray-300 border p-2 text-left w-1/10">
                            //                             Offset
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             00
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             01
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             02
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             03
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             04
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             05
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             06
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             07
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             08
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             09
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0A
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0B
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0C
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0D
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             "0E"
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0F
                            //                         </th>
                            //                     </tr>
                            //                 </thead>
                            //                 <tbody>
                            //                     <tr>
                            //                         <td class="border p-2 bg-gray-200">000000</td>
                            //                         <td class="border p-2">41</td>
                            //                         <td class="border p-2">42</td>
                            //                         <td class="border p-2">4F</td>
                            //                         <td class="border p-2">41</td>
                            //                         <td class="border p-2">42</td>
                            //                         <td class="border p-2">4F</td>
                            //                         <td class="border p-2">41</td>
                            //                         <td class="border p-2">42</td>
                            //                         <td class="border p-2">4F</td>
                            //                         <td class="border p-2">41</td>
                            //                         <td class="border p-2">42</td>
                            //                         <td class="border p-2">4F</td>
                            //                         <td class="border p-2">4F</td>
                            //                         <td class="border p-2">41</td>
                            //                         <td class="border p-2">42</td>
                            //                         <td class="border p-2">4F</td>
                            //                     </tr>
                            //                     <tr>
                            //                         <td class="border p-2 bg-gray-200">000010</td>
                            //                         <td class="border p-2">11</td>
                            //                         <td class="border p-2">F2</td>
                            //                         <td class="border p-2">FF</td>
                            //                     </tr>
                            //                 </tbody>
                            //             </table>
                            //         </div>
                            //     </div>
                            //     <div class="w-46/100">
                            //         <div class="overflow-x-auto m-4">
                            //             <table class="w-full border-collapse table-fixed">
                            //                 <thead>
                            //                     <tr>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             00
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             01
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             02
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             03
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             04
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             05
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             06
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             07
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             08
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             09
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0A
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0B
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0C
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0D
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             "0E"
                            //                         </th>
                            //                         <th class="sticky top-0 bg-gray-200 border p-2 text-left">
                            //                             0F
                            //                         </th>
                            //                     </tr>
                            //                 </thead>
                            //                 <tbody>
                            //                     <tr>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">F</td>
                            //                         <td class="border p-2">U</td>
                            //                         <td class="border p-2">N</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">c</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">.</td>
                            //                         <td class="border p-2">v</td>
                            //                         <td class="border p-2">i</td>
                            //                         <td class="border p-2">i</td>
                            //                         <td class="border p-2">.</td>
                            //                     </tr>
                            //                     <tr>
                            //                         <td class="border p-2">11</td>
                            //                         <td class="border p-2">F2</td>
                            //                         <td class="border p-2">FF</td>
                            //                     </tr>
                            //                 </tbody>
                            //             </table>
                            //         </div>
                            //     </div>
                            </Show>
                        </div>
                        <div class="w-full h-2/5 mt-4 border-t border-gray-200 dark:border-gray-800">
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
                            <div class="text-sm p-4 flex flex-row">
                                <div class="w-7/100 flex">
                                    <div class="h-4">
                                        <h3>Offset</h3>
                                    </div>
                                </div>
                                <div class="w-23/100 flex">
                                    <div class="h-4">
                                        <h3>HEX</h3>
                                    </div>
                                </div>
                                <div class="w-33/100">
                                    <div class="h-4">
                                        <h3>Assembly</h3>
                                    </div>
                                    <Show when=move || !unified_data().is_empty()>
                                        <pre class="border border-gray-200 rounded p-2 bg-gray-100 font-mono text-xs md:text-md xl:text-lg overflow-x-scroll">
                                            {move || disassembled_data().clone()}
                                        </pre>
                                    </Show>
                                </div>
                                <div class="w-33/100">
                                    <div class="h-4">
                                        <h3>Hint</h3>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="offset-bar w-1/100 bg-gray-300 dark:bg-gray-800 p-2 overflow-auto">
                    <div class="w-full h-full bg-gray-400 dark:bg-gray-700 rounded-md"></div>
                    test
                </div>
            </main>
        </div>
    }
}
