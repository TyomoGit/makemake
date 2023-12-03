use std::{env::args, fs, io::Error};

mod material;
mod scanner;

fn main() -> Result<(), Error> {
    let elements = fs::read_dir(".")?;

    let mut materials = Vec::new();

    for element in elements {
        let element = element?;

        if element.file_type()?.is_dir() { continue; }
        let file_name = element.file_name();
        if !file_name.to_str().unwrap().ends_with(".c") {
            continue; 
        }

        let material = scanner::scan(file_name.to_str().unwrap())?;
        materials.push(material);
    }

    let binary_name = args().nth(1).unwrap_or("main.out".to_string());
    let mut makefile = String::new();
    let mut all_objects = String::new();
    for material in &materials {
        all_objects.push_str(&format!("{} ", material.object_name()));
    }

    makefile.push_str(&format!("{}: {}\n", binary_name, all_objects));
    makefile.push_str(&format!("\tgcc {} -o {}\n\n", all_objects, binary_name));

    for material in &materials {
        makefile.push_str(&format!(
            "{}: {} {}\n",
            material.object_name(),
            material.c_name(),
            material.dependencies_name()
        ));
        makefile.push_str(&format!("\t{}\n\n", material.gcc_command()));
    }

    makefile.push_str(&format!("run: {}\n", binary_name));
    makefile.push_str(&format!("\t./{}\n\n", binary_name));

    makefile.push_str("clean:\n");
    makefile.push_str("\trm -f *.o *.out\n");

    fs::write("Makefile", makefile)?;

    Ok(())
}
