// extern crate mipha;
//
// use mipha::{QrCode, QrCodeEcc};
//
// // Prints the given QrCode object to the console.
// fn print_qr(qr: &QrCode) {
//     let border: i32 = 4;
//     for y in -border..qr.size() + border {
//         for x in -border..qr.size() + border {
//             let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
//             print!("{0}{0}", c);
//         }
//         println!();
//     }
//     println!();
// }
//
// fn main() {
//     let text: &'static str = "Hello, world!"; // User-supplied Unicode text
//     let errcorlvl: QrCodeEcc = QrCodeEcc::Low; // Error correction level
//
//     // Make and print the QR Code symbol
//     let qr: QrCode = QrCode::encode_text(text, errcorlvl).unwrap();
//     print_qr(&qr);
//     println!("{}", qr.to_svg_string(4));
// }
