#![windows_subsystem = "windows"]

use aes::cipher::{
    generic_array::{typenum::U16, GenericArray},
    BlockDecrypt, BlockEncrypt, KeyInit,
};
use aes::Aes128;
use eframe::egui;

struct MyApp {
    password: String,
    plaintext: String,
    depassword: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            password: "Daniel".to_owned(),
            plaintext: "SRC kryptering".to_string(),
            depassword: "Daniel er sej".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AES DEMO");

            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
            ui.separator();

            ui.horizontal(|ui| {
                let password_label = ui.label("Dit kodeord (maks 16 bogstaver): ");
                ui.text_edit_singleline(&mut self.password)
                    .labelled_by(password_label.id);
            });

            ui.horizontal(|ui| {
                let text_label = ui.label("Din tekst: ");
                ui.text_edit_multiline(&mut self.plaintext)
                    .labelled_by(text_label.id);
            });

            ui.separator();

            let key = string_into_generic_array(self.password.clone());
            let cipher = Aes128::new(&key);

            let mut blocks = split_into_generic_arrays(self.plaintext.clone());

            cipher.encrypt_blocks(&mut blocks);

            let ciphertext = generic_arrays_to_string(blocks.clone());
            ui.label("Enkryptered tekst:");
            ui.label(ciphertext);

            ui.separator();
            ui.horizontal(|ui| {
                let text_label = ui.label("Dekrypterings kodeord: ");
                ui.text_edit_singleline(&mut self.depassword)
                    .labelled_by(text_label.id);
            });

            let new_key = string_into_generic_array(self.depassword.clone());
            let new_cipher = Aes128::new(&new_key);

            new_cipher.decrypt_blocks(&mut blocks);
            let plaintext = generic_arrays_to_string(blocks);

            ui.label("Dekryptered tekst:");
            ui.label(plaintext);
        });
    }
}

fn string_into_generic_array(input: String) -> GenericArray<u8, U16> {
    let mut buffer = [0u8; 16];
    let bytes = input.as_bytes();
    let len = bytes.len().min(16);
    buffer[..len].copy_from_slice(&bytes[..len]);

    GenericArray::clone_from_slice(&buffer)
}

fn split_into_generic_arrays(input: String) -> Vec<GenericArray<u8, U16>> {
    let bytes = input.as_bytes();
    let mut arrays = Vec::new();

    for chunk in bytes.chunks(16) {
        let mut buffer = [0u8; 16];
        let len = chunk.len().min(16);
        buffer[..len].copy_from_slice(&chunk[..len]);
        arrays.push(GenericArray::clone_from_slice(&buffer));
    }

    arrays
}

fn generic_arrays_to_string(arrays: Vec<GenericArray<u8, U16>>) -> String {
    let mut bytes = Vec::new();

    for array in arrays {
        bytes.extend_from_slice(&array[..]);
    }

    String::from_utf8_lossy(&bytes).to_string()
}

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "AES demostration",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
    .unwrap();
}
