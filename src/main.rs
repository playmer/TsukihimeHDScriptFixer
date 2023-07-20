

const IMAGE_EXTENSIONS : [&'static str; 4] = [".jpg", ".jpeg", ".bmp", ".png"];

fn get_image_path(line : String) -> Option<String>
{
    for extension in IMAGE_EXTENSIONS 
    {
        if let Some(index) = line.find(extension)
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
    let script_shift_jis_bytes = std::fs::read("original_HD_nscript.txt").unwrap();
    
    let (res, _enc, errors) = encoding_rs::SHIFT_JIS.decode(&script_shift_jis_bytes);
    if errors {
        eprintln!("Failed");
    } else {

        for line in  res.to_string().lines()
        {
            let line = line.to_lowercase();
            if !line.starts_with(";")
            {
                if let Some(image_path) = get_image_path(line)
                {
                    println!("{}", image_path);
                }
            }
        }
    }   
}
