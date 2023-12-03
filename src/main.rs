use std::{env::args, fs, io::Error};

use material::Material;

mod material;
mod scanner;

fn main() -> Result<(), Error> {
    let materials = collect_materials()?;

    let binary_name = args().nth(1).unwrap_or("main.out".to_string());
    let all_objects = str_all_objects(&materials);
    let mut makefile = String::new();

    makefile.push_str(&makefile_binary(&binary_name, &all_objects));
    makefile.push_str(&makefile_objects(&materials));
    makefile.push_str(&makefile_utils(&binary_name));

    fs::write("Makefile", makefile)?;

    Ok(())
}

fn collect_materials() -> Result<Vec<Material>, Error> {
    let elements = fs::read_dir(".")?;

    let mut materials = Vec::new();

    for element in elements {
        let element = element?;

        if element.file_type()?.is_dir() {
            continue;
        }
        let file_name = element.file_name();
        if !file_name.to_str().unwrap().ends_with(".c") {
            continue;
        }

        let material = scanner::scan(file_name.to_str().unwrap())?;
        materials.push(material);
    }

    Ok(materials)
}

fn str_all_objects(materials: &[Material]) -> String {
    let mut all_objects = String::new();
    for material in materials {
        all_objects.push_str(&format!("{} ", material.object_name()));
    }
    all_objects
}

fn makefile_binary(binary_name: &str, all_objects: &str) -> String {
    format!("{}: {}\n\n", binary_name, all_objects)
}

fn makefile_objects(materials: &[Material]) -> String {
    let mut result = String::new();
    for material in materials {
        result.push_str(&format!(
            "{}: {} {}\n",
            material.object_name(),
            material.c_name(),
            material.dependencies_name()
        ));
        result.push_str(&format!("\t{}\n\n", material.gcc_command()));
    }

    result
}

fn makefile_utils(binary_name: &str) -> String {
    format!("run: {}\n\t./{}\n\nclean:\n\trm -f *.o *.out\n",
        binary_name, binary_name
    )
}
