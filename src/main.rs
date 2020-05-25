extern crate walkdir;

use walkdir::WalkDir;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

fn main() {
    let x = get_svg_files();
    let y = create_svg_wrapper(x.expect("lol"));
    save_combined_svg(y).unwrap();
}

fn get_svg_files() -> Result<String, String> {
    println!("getfiles run");
    let mut svg_combined_content = String::from("");
    for entry in WalkDir::new(".")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok()){
        let f_name = entry.file_name().to_string_lossy();
        let f_path = String::from(entry.path().to_string_lossy());
        if f_name.ends_with(".svg"){
            let svg_content = get_svg_content(f_path);
            let svg_symbol = process_svg(svg_content, f_name.to_string());
            svg_combined_content.push_str(&svg_symbol);
        }
    }
    if svg_combined_content != ""{
        Ok(svg_combined_content)
    }
    else{
        let error = String::from("error");
        Err(error)
    }
   
}

fn create_svg_wrapper(svg_content:String) -> String{
    let svg_template_start = String::from("<svg aria-hidden=\"true\" focusable=\"false\" xmlns=\"http://www.w3.org/2000/svg\">");
    let svg_template_end = String::from("</svg>");
    let svg = &format!("{}{}{}", svg_template_start, svg_content, svg_template_end);
    return svg.to_string();
}

fn get_svg_content(file_path:String) -> String{
    let mut file = File::open(file_path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}

fn process_svg(svg_content:String, svg_file_name:String) -> String{
    let svg_start = String::from("<svg"); 
    let svg_end = String::from("</svg>"); 
    let xmlns = String::from("xmlns=\"http://www.w3.org/2000/svg\""); 
    let regex_id = Regex::new(r#"(id="([^"]|")*")"#).unwrap();
    let svg_name = svg_file_name.replace(".svg", "");
    let symbol_start_with_id = &format!("{}{}{}{}", "<symbol", " id=\"", svg_name, "\"");
    let remove_svg_id = regex_id.replace_all(&svg_content, "");

    let x = remove_svg_id.replace(&xmlns, "");
    let y = x.replace(&svg_start, symbol_start_with_id);
    let z = y.replace(&svg_end, "</symbol>");
    return z.to_string();
}

fn save_combined_svg(svg_source:String)-> std::io::Result<()> {
    let mut file = File::create("./combined.txt")?;
    println!("{}",svg_source);
    file.write_all(svg_source.as_bytes())?;
    Ok(())
}
