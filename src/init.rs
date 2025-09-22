use serde::Serialize;
use std::fs::{self, File};

pub fn setup(cmdlang: String, projectname: String) {
    setupmakefile(&cmdlang, &projectname);
    let _ = forge_toml(&projectname);
    mainfile(&cmdlang);
}

fn setupmakefile(cmdlang: &String, projectname: &String) {
    let _ = fs::create_dir("src/");
    let _ = fs::create_dir("include/");
    let _ = File::create("Makefile");
    let _ = File::create("forge.toml");
    if cmdlang == "c++" {
        let _ = File::create("src/main.c++");
    } else {
        let _ = File::create("src/main.c");
    }

    let (compiler, lang): (&str, &str) = match cmdlang.as_str() {
        "c" => ("gcc", "c"),
        "c++" => ("g++", "c++"),
        _ => ("gcc", "c"),
    };

    let make = format!(
        "cc ={}
lang = {}
includes = -Iinclude

src = $(shell find src -name \"*.$(lang)\")
target = bin/{}

$(target):$(src)
	@mkdir bin/
	$(cc) $(includes) -o $@ $^

.PHONY:run
run:$(target)
	./$(target)
	@rm -fr bin/

.PHONY:build
build:$(target)

.PHONY:clean
clean:
	rm -fr bin/

    ",
        compiler, lang, projectname
    );
    let _ = fs::write("Makefile", make);
}

#[derive(Serialize)]
struct Project {
    name: String,
    version: String,
    debug: bool,
}

#[derive(Serialize)]
struct Config {
    project: Project,
}
fn forge_toml(projectname: &String) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        project: Project {
            name: projectname.to_string(),
            version: "0.1.0".to_string(),
            debug: true,
        },
    };

    let toml_string = toml::to_string(&config).unwrap();

    fs::write("forge.toml", toml_string)?;

    Ok(())
}

fn mainfile(cmdlang: &String) {
    if cmdlang == "c++" {
        let maincontent = "#include <iostream>
using namespace std;

int main(){
cout << \"hello world\" << endl;
return 0;
}";
        let _ = fs::write("src/main.c++", maincontent);
    } else {
        let maincontent = "#include <stdio.h>

int main() {
    printf(\"Hello, World!\\n\");
    return 0;
}
       ";
        let _ = fs::write("src/main.c", maincontent);
    }
}
