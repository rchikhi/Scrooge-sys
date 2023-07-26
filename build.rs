use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let c_src_path = Path::new("Scrooge");

    Command::new("cp")
        .current_dir(&c_src_path)
        .arg("Makefile")
        .arg("Makefile.old")
        .output()
        .expect("Failed to backup Scrooge makefile.");

    Command::new("sh")
            .arg("-c")
            .arg("echo 'genasm_cpu: $(SRC)/genasm_cpu.cpp' >> Makefile")
            .current_dir(&c_src_path)
            .output()
            .unwrap();

    Command::new("sh")
            .arg("-c")
            .arg("echo ' $(CXX) -c $(SRC)/genasm_cpu.cpp  $(CXX_FLAGS) -lpthread -lstdc++fs -fopenmp' >> Makefile")
            .current_dir(&c_src_path)
            .output()
            .unwrap();

    // build the library
    Command::new("make")
        .arg("genasm_cpu")
        .current_dir(&c_src_path)
        .output()
        .expect("Failed to build genasm_cpu.");

    //eprintln!("ls {:?}",String::from_utf8(Command::new("ls").current_dir(&c_src_path).output().unwrap().stdout));

    let file= "genasm_cpu.o";
    Command::new("cp")
        .arg(file)
        .arg(&out_dir)
        .current_dir(&c_src_path)
        .output()
        .expect("Failed to copy scrooge object files.");

    // package all .o files into a static library
    Command::new("sh")
        .arg("-c")
        .arg("ar rcs libscrooge.a *.o")
        .current_dir(&out_dir)
        .output()
        .unwrap();

    // clean up the temporary build files
    Command::new("make")
        .current_dir(&c_src_path)
        .arg("clean")
        .output()
        .expect("Failed to clean up scrooge build files.");

    Command::new("mv")
        .current_dir(&c_src_path)
        .arg("Makefile.old")
        .arg("Makefile")
        .output()
        .expect("Failed to restore scrooge makefile.");


    // let cargo know that it can find the file in the out directory
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=scrooge");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
