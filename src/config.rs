use std::fs;

// Structs
#[derive(Debug)]
pub struct Configuration {
    pub file_extensions: Vec<String>,
    pub file_names: Vec<String>,
    pub dirs: Vec<String>,
}

// Impls
impl Configuration {
    pub fn new(args: Vec<String>) -> Self {
        let mut config = Self {
            file_extensions: vec![],
            file_names: vec![],
            dirs: vec![],
        };

        config.parse_args(args);
        config.verify_files_exist();

        config
    }

    fn parse_args(&mut self, args: Vec<String>) {
        for arg in args {
            //println!("Arg = {arg}");
            let chars: Vec<char> = arg.chars().collect();
            if chars[0] != '-' || chars[2] != '=' || !['d', 'e', 'f'].contains(&chars[1]) {
                eprintln!("ERROR: '{arg}' is not a valid configuration argument. 
                    \nUsage: 
                    \nSpecify one or more files: `-f=/path/file.rs,/path/otherfile.rs`
                    \nSpecify a directory alongside file endings: `-d=/dir/project1,/dir/project2`  `-e=rs,js,html`"
                );

                panic!("Invalid arguments given.");
            }

            let rhs = chars[3..]
                .iter()
                .collect::<String>()
                .split(",")
                .map(|s| s.into())
                .collect::<Vec<String>>();

            match chars[1] {
                'd' => self.dirs.extend(rhs),
                'e' => self.file_extensions.extend(rhs),
                'f' => self.file_names.extend(rhs),
                _ => {}
            }
        }
    }

    fn verify_files_exist(&mut self) {
        self.file_names = self
            .file_names
            .iter()
            .filter(|filename| {
                if let Ok(data) = fs::metadata(filename) {
                    if data.is_file() {
                        return true;
                    }
                }

                eprintln!("WARNING: File `{}` does not exist, skipping.", filename);
                false
            })
            .map(|s| s.into())
            .collect::<Vec<String>>();
    }

    fn find_files_in_dir_with_ext(&mut self) {}
}
