

const IMAGE_EXTENSIONS : [&'static str; 4] = [".jpg", ".jpeg", ".bmp", ".png"];

fn get_image_path(line : &str) -> Option<String>
{
    for extension in IMAGE_EXTENSIONS 
    {
        let line_lowercase = line.to_lowercase();
        if let Some(index) = line_lowercase.find(extension)
        {
            let first_part = line.split_at(index).0;
            
            let semicolon_index = first_part.rfind(";");
            let quote_index = first_part.rfind("\"").unwrap();

            let start_index = 1 + if semicolon_index.is_some() && semicolon_index.unwrap() > quote_index {
                semicolon_index.unwrap()
            }
            else {
                quote_index
            };
            //let start_index = 
            let end_index = index + extension.len();

            return Some(line[start_index..end_index].to_string());
        }
    }
    
    return None;
}

fn main() {
    let script_shift_jis_bytes = std::fs::read("edited_16x9_nscript.txt").unwrap();
    
    let (res, _enc, errors) = encoding_rs::SHIFT_JIS.decode(&script_shift_jis_bytes);
    if errors {
        eprintln!("Failed");
    } else {

        let mut new_script = String ::new();

        for line in  res.to_string().split("\r\n")
        {
            if !line.starts_with(";")
            {
                if let Some(image_path) = get_image_path(&line)
                {
                    let texture_sd_path = &image_path;
                    let texture_hd_path = format!("HD_{image_path}");
                    let texture_16x9_path = format!("16x9_{image_path}");

                    new_script.push_str(format!("if %tex_SD==1 {}\r\n", line.replace(&image_path, texture_sd_path)).as_str());
                    new_script.push_str(format!("if %tex_HD==1 {}\r\n", line.replace(&image_path, &texture_hd_path)).as_str());
                    new_script.push_str(format!("if %tex_16x9==1 {}\r\n", line.replace(&image_path, &texture_16x9_path)).as_str());
                }
                else
                {
                    new_script.push_str(format!("{}\r\n", line).as_str());
                }
            }
            else 
            {
                new_script.push_str(format!("{}\r\n", line).as_str());
            }
        }

        let (bytes, encoding, success) = encoding_rs::SHIFT_JIS.encode(&new_script);
        std::fs::write("new_script.txt", bytes).unwrap();
    }   
}
