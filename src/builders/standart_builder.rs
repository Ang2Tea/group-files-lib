use std::{fs, env, path::Path};

use crate::sorting::sorting_files::{SortingFile, SortingFiles};

pub struct SortingFielsBuilder{
    input: String,
    output : String,
    check_output : bool,
    force : bool,
    show_hidden : bool,
}

impl SortingFielsBuilder {

    fn check_dir(dir: &String) -> bool {
        if !Path::new(dir).exists() { return false; }
        true
    }
    
    fn get_files(&self) -> Vec<SortingFile> {
        // Читает директори и вызывает ошибку если ее нет
        let entries = fs::read_dir(&self.input).unwrap_or_else(|e| {
            panic!("Error reading directory: {}", e);
        });

        // Переменная для результатов
        let mut result = Vec::<SortingFile>::new();

        for entry in entries.filter_map(Result::ok) {
            let file_path = entry.path();
            let file_name_str = file_path.to_string_lossy();
    
            if let Ok(file_type) = entry.file_type() {
                // Выбирает файлы по условию это видимый файл или при show_hidden выбирает все файлы
                if (self.show_hidden || !file_name_str.starts_with('.')) && file_type.is_file() {
                    // Создает временую переменную для Sorting file
                    let temp_result = match SortingFile::new(
                        &file_path.to_string_lossy().to_string(),
                         &self.output, 
                         self.force) {
                            Ok(r) => r,
                            Err(e) => {
                                panic!("{}", e);
                            }
                    };
                    result.push(temp_result);
                }
            }
        }
        result
    }

    /// Method set for field input.
    ///
    /// # Arguments
    ///
    /// * `value` - argument to write
    ///
    /// # Returns
    ///
    /// The same object to build builder.
    pub fn set_input(&mut self, value : Option<String>)-> &mut Self{
        if let Some(path) = value{
            self.input = path;
        }
        
        if self.check_output {
            self.output = self.input.clone();
            self.check_output = false;
        }

        self
    }

    /// Method set for field output
    ///
    /// # Arguments
    ///
    /// * `value` - argument to write
    ///
    /// # Returns
    ///
    /// The same object to build builder
    pub fn set_output(&mut self, value : Option<String>) -> &mut Self{
        if let Some(path) = value{
            self.output = path;
        }

        self
    }

    /// Method set for field showhidden
    ///
    /// # Arguments
    ///
    /// * `value` - argument to write
    ///
    /// # Returns
    ///
    /// The same object to build builder
    pub fn set_show_hidden(&mut self, value : bool) -> &mut Self{
        self.show_hidden = value;

        self
    }

    /// Method set for field force mode moved files.
    ///
    /// # Arguments
    ///
    /// * `value` - argument to write
    ///
    /// # Returns
    ///
    /// The same object to build builder
    pub fn set_force(&mut self, value : bool) -> &mut Self{
        self.force = value;

        self
    }

    /// Method build SortingFiles struct
    ///
    /// # Returns
    ///
    ///Assembly by arguments from the builder structure SortingFiles
    pub fn build(&self) -> Result<SortingFiles, String>{
        if !Self::check_dir(&self.input) {
            let err_message = format!("Dir {} not exists", &self.input);
            return Err(err_message);
        }
        if !Self::check_dir(&self.output) {
            let err_message = format!("Dir {} not exists", &self.output);
            return Err(err_message);
        }

        let files = self.get_files();

        Ok(SortingFiles::new( &self.output, files))
    }

    /// Method create new builder with default arguments
    ///
    /// # Returns
    ///
    /// New SortingFielsBuilder
    pub fn new() -> Self{
        // TODO Посмотреть получение корневой директории
        let path_str = env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|e| {
            println!("Failed to get the current directory: {}", e);
            std::process::exit(1);
        });

        SortingFielsBuilder{
            input : path_str.clone(),
            output : path_str.clone(),
            check_output: true,
            force : false,
            show_hidden : false
        }
    }
}
