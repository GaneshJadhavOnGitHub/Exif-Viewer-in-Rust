use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use exif;

fn main(){
    // Images whose EXIF data is to be extracted.
    //let file_name = "Konica_Minolta.jpg".to_string();
    let file_name = "Sebastien_Jefferies.jpg".to_string();
    
    let info = print_exif_data(file_name);
    if info.is_err(){
        println!("ERROR in EXIF VIEWER"); 
    }
}
/// Function to print EXIF Data
fn print_exif_data(file_name: String) -> Result<(), exif::Error> {
    println!("--  EXIF VIEWER -- ");
    println!("-------------------- ");

    // Replace image file extension with txt. This txt file will show EXIF Data for that image.
    let file_name_length =  file_name.len();
    let mut file_name_replaced = file_name.clone();
    let mut file_name_replaced_to_open_file = file_name.clone();
    file_name_replaced.replace_range((file_name_length-3)..file_name_length, "txt");
    file_name_replaced_to_open_file.replace_range((file_name_length-3)..file_name_length, "txt");
    
    let path = Path::new(&file_name);
    let file = std::fs::File::open(path);
    if file.is_ok(){
        
        //Create text file with same name as image.
        let _create_file = File::create(file_name_replaced).expect("Error encountered while creating file!");
        
        // Get EXIF Data.
        let file = file.unwrap();
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader);
         if exif.is_ok(){

            // Open the file and write EXIF Data. 
            let mut data_file = OpenOptions::new()
            .append(true)
            .open(file_name_replaced_to_open_file)
            .expect("Cannot open file");

            // Write Header Text in the file.      
            let header_text = "**  Exif Data for ".to_string()
                                     + &file_name.clone().to_string()
                                     + &"  **" + "\n_________________________________________________________________";
            data_file.write(header_text.as_bytes()).expect("Write failed");
            println!("{}", header_text);
            
            // Skip some tags.
            let exif = exif.unwrap();
            for f in exif.fields() {
                //Following three tags are not displayed as they show hexaDecimal data.
                let tag_to_omit1: &str = "MakerNote";
                let tag_to_omit2: &str = "UserComment";
                let tag_to_omit3: &str = "Tag(Tiff, 50341)";
                if f.tag.to_string().as_str().eq(tag_to_omit1) || 
                   f.tag.to_string().as_str().eq(tag_to_omit2) ||
                   f.tag.to_string().as_str().eq(tag_to_omit3)
                {
                } 
                else
                {
                    // Write EXIF Data to file.
                    let output = "\n".to_string() 
                                         + &f.tag.to_string() 
                                        + &" : ".to_string() 
                                        + &f.display_value().with_unit(&exif).to_string();
                    println!("{}", output);
                    data_file.write(output.as_bytes()).expect("Write failed");
                }  
            }
            // Write Footer Text.
            let footer_text = "\n ** End ** ".to_string();
            data_file.write(footer_text.as_bytes()).expect("Write failed");

            // Flush the output stream after writing EXIF Data in the file.
            // This ensures that buffer get written to the file.
            let _flushed = data_file.flush();
            println!("-----------------------------------------------------------------");
        }
        else {
            // Error while reading EXIF Data such that EXIF Data not present.
             return Err(exif.err().unwrap());
        }
    }
    else { 
          println!("Error : File Not Found!");
       }
    
   Ok(())
}