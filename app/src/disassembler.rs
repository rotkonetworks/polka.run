use leptos::*;
use polkavm::ProgramBlob;
use crate::file_upload::FileUploadComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
enum MenuItemType {
    RegularItem,
    SubMenu(Vec<MenuItem>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MenuItem {
    label: String,
    item_type: MenuItemType,
    action: Option<String>,
}

struct MainMenu {
    items: Vec<MenuItem>,
}



#[component]
fn MenuButton(item: MenuItem) -> impl IntoView {
    view! {
        <div
            role="menuitem"
            class="flex items-center space-x-1 px-2 py-1 rounded-md border hover:bg-gray-200 dark:hover:bg-gray-700"
            tabindex="-1"
            style="outline:none"
        >
            <div class="w-3 h-3 rounded-full bg-red-500"></div>
            <div class="text-sm font-medium text-gray-700 dark:text-gray-200">
                {item.label}
            </div>
        </div>
    }
}


#[component]
pub fn Disassembler() -> impl IntoView {

    pub struct DisassemblyLine {
        pub offset_part: String,
        pub hex_part: String,
        pub text_part: String,
    }

    fn unified_representation(data: &[u8]) -> Vec<DisassemblyLine> {
        let mut offset = 0usize; // Initialize the offset outside the map

        data.chunks(8)
            .map(|chunk| {
                let mut hex_part = String::with_capacity(23);
                let mut text_part = String::with_capacity(8);

                for &byte in chunk {
                    // Write the hex representation directly into hex_part.
                    use std::fmt::Write;
                    write!(hex_part, "{:02x} ", byte).expect("Writing to a String should never fail");

                    // Append ASCII representation or '.' to text_part.
                    text_part.push(if (32..=126).contains(&byte) { byte as char } else { '.' });
                }

                // Format the offset as a 6-digit hexadecimal number.
                let offset_hex = format!("{:06x}", offset);

                // Increment the offset by the chunk size (8 bytes).
                offset += 8;

                // Trim the trailing space from the hex_part and pad if necessary.
                let hex_part = hex_part.trim_end().to_string();
                let hex_part_padded = format!("{:23}", hex_part);

                // Pad text_part if necessary.
                let text_part_padded = format!("{:<8}", text_part);

                DisassemblyLine {
                    offset_part: offset_hex,
                    hex_part: hex_part_padded,
                    text_part: text_part_padded,
                }
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


    let title = format!("polkavm disassembler").to_string();

    // Improved state management with signals
    let (disassembled_data, set_disassembled_data) = create_signal(Vec::<T>::new());


    pub fn MenuButton() -> impl IntoView {
        view! {
            <div
                role="menuitem"
                class="flex items-center space-x-1 px-2 py-1 rounded-md border hover:bg-gray-200 dark:hover:bg-gray-700"
                tabindex="-1"
                style="outline:none"
            >
                <div class="w-3 h-3 rounded-full bg-red-500"></div>
                <div class="text-sm font-medium text-gray-700 dark:text-gray-200">
                    {label}
                </div>
            </div>
        }
    }

    #[component]
    fn MainPageLayout() -> impl IntoView {
        view! {
            <div class="h-screen w-full flex flex-col">
                <header class="flex h-16 w-full items-center px-4 md:px-6 bg-gray-100 dark:bg-gray-800">
                    <div
                    >
                    <nav
                        role="menubar"
                        class="flex h-10 items-center space-x-1 rounded-md border bg-background p-1"
                        tabindex="0"
                        data-orientation="horizontal"
                        style="outline:none"
                    >
                        { for menu.items.iter().map(|item| view! { <MenuButton item=item /> }) }
                    </nav>
                        <MenuButton label="File"/>
                        <MenuButton label="Settings"/>
                        <MenuButton label="View"/>
                        <MenuButton label="Compare"/>
                        <MenuButton label="Info"/>
                    </div>
                </header>
                <main class="flex flex-1 w-full">
                    incoming
                </main>
            </div>
        }
    }
}
