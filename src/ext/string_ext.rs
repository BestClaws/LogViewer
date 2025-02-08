pub trait StringExt {
    fn hex_color(&self) -> egui::Color32;
}


impl StringExt for str {
    fn hex_color(&self) -> egui::Color32 {
       egui::Color32::from_hex(self).unwrap() 
    } 
}