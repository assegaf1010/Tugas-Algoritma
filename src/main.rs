use std::io;
use std::collections::HashMap;

// Struct untuk item inventaris
#[derive(Debug)]
struct InventoryItem {
    code: String,
    name: String,
    quantity: u32,
    // Tambahkan field lain jika diperlukan
}

// Struct aplikasi inventaris
struct InventoryApp {
    inventory: HashMap<String, InventoryItem>,
}

impl InventoryApp {
    fn new() -> InventoryApp {
        InventoryApp {
            inventory: HashMap::new(),
        }
    }

    // Menambahkan item ke inventaris
    fn add_item(&mut self, code: String, name: String, quantity: u32) {
        let item = InventoryItem {
            code: code.clone(),
            name: name.clone(),
            quantity,
        };
        self.inventory.insert(code, item);
    }

    // Melihat semua item di inventaris
    fn view_inventory(&self) {
        println!("Inventaris:");
        println!("-----------------------------------------");
        println!("{:<10} {:<20} {:<10}", "Kode", "Nama", "Jumlah");
        println!("-----------------------------------------");

        for (_, item) in &self.inventory {
            println!("{:<10} {:<20} {:<10}", item.code, item.name, item.quantity);
        }

        println!("-----------------------------------------");
    }

    // Mengedit item di inventaris
    fn edit_item(&mut self, code: &str, new_quantity: u32) {
        if let Some(item) = self.inventory.get_mut(code) {
            item.quantity = new_quantity;
            println!("Item dengan kode {} diubah menjadi jumlah: {}", code, new_quantity);
        } else {
            println!("Item dengan kode {} tidak ditemukan.", code);
        }
    }

    // Menghapus item dari inventaris
    fn delete_item(&mut self, code: &str) {
        if let Some(_) = self.inventory.remove(code) {
            println!("Item dengan kode {} dihapus dari inventaris.", code);
        } else {
            println!("Item dengan kode {} tidak ditemukan.", code);
        }
    }
}

fn main() {
    let mut inventory_app = InventoryApp::new();

    println!("=== APLIKASI INVENTORY ===");

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah item");
        println!("2. Lihat inventaris");
        println!("3. Edit item");
        println!("4. Hapus item");
        println!("5. Keluar");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Gagal membaca baris.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Masukkan kode item:");
                let mut code = String::new();
                io::stdin().read_line(&mut code)
                    .expect("Gagal membaca baris.");
                let code = code.trim().to_string();

                println!("Masukkan nama item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name)
                    .expect("Gagal membaca baris.");
                let name = name.trim().to_string();

                println!("Masukkan jumlah item:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity)
                    .expect("Gagal membaca baris.");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Masukkan angka yang valid!");
                        continue;
                    }
                };

                inventory_app.add_item(code, name, quantity);
            }
            2 => {
                inventory_app.view_inventory();
            }
            3 => {
                println!("Masukkan kode item yang ingin diubah:");
                let mut code = String::new();
                io::stdin().read_line(&mut code)
                    .expect("Gagal membaca baris.");
                let code = code.trim();

                println!("Masukkan jumlah baru:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity)
                    .expect("Gagal membaca baris.");
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Masukkan angka yang valid!");
                        continue;
                    }
                };

                inventory_app.edit_item(code, quantity);
            }
            4 => {
                println!("Masukkan kode item yang ingin dihapus:");
                let mut code = String::new();
                io::stdin().read_line(&mut code)
                    .expect("Gagal membaca baris.");
                let code = code.trim();

                inventory_app.delete_item(code);
            }
            5 => {
                println!("Keluar dari aplikasi.");
                break;
            }
            _ => {
                println!("Pilihan tidak valid, coba lagi.");
            }
        }
    }
}
