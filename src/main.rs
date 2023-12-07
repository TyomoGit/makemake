use std::{
    env::args,
    fs,
    io::Error,
    process::{exit, Command},
};

use material::Material;

mod material;
mod scanner;

const BINARY_NAME: &str = "a.out";

fn main() -> Result<(), Error> {
    let materials = collect_materials()?;

    let all_objects = str_all_objects(&materials);
    let mut makefile = String::new();

    makefile.push_str(&makefile_binary(BINARY_NAME, &all_objects));
    makefile.push_str(&makefile_objects(&materials));
    makefile.push_str(&makefile_utils(BINARY_NAME));

    fs::write("Makefile", makefile)?;

    if let Some(make_label) = args().nth(1) {
        let code = execute_make(&make_label);
        if code != 0 {
            exit(code);
        }
    }

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
    format!("{0}: {1}\n\tgcc {1}-o {0}\n\n", binary_name, all_objects)
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
    format!(
        "run: {}\n\t./{}\n\nclean:\n\trm -f *.o *.out\n",
        binary_name, binary_name
    )
}

fn execute_make(label: &str) -> i32 {
    let command = Command::new("make")
        .arg(label)
        .output()
        .expect("failed to execute process");
    if command.stderr.is_empty() {
        print!("{}", String::from_utf8_lossy(&command.stdout));
    } else {
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    command.status.code().unwrap()
}
