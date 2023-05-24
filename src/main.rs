mod websocket;
use printers;

fn main() {
    let file_path = "assets/coba.pdf";

    let printers = printers::get_printers();

    let printer = match printers.get(0) {
        Some(p) => p,
        None => {
            println!("printer tidak ditemukan");
            return;
        }
    };

    printers::print_file(&printer, file_path);
}
