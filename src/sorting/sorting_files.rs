use std::{path::Path, fs};

const INDEFINITE_FILE_NAME: &str = "indefinite";
const OPERATING_SYSTEM_SLASH: &str = if cfg!(windows) {  "\\" } else {  "/" };

pub struct SortingFile{
    file_path: String,
    name: String,
    extension: String,
}

impl SortingFile {
    /// Method for get current name if file has in output dir
    ///
    /// # Arguments
    ///
    /// * `file_path` - argument path for file
    /// * `output_path` - output dir for file
    /// * `force` - force mode file movement
    ///
    /// # Returns
    /// 
    /// Return new file name or Err in enum Result<String, String> 
    fn current_file_name(file_path: &Path, output_path: &Path, force: bool) -> Result<String, String> {
        // Получение имени
        let source_file_name = file_path
            .file_name()
            .ok_or("Invalid source file path")?;
    
        // Переменная для нового имени
        let mut new_file_name = source_file_name
        .to_string_lossy()
        .to_string();
        // Итератор для нового имени
        let mut counter = 1;
    
        // Путь для нового имени
        let mut destination_path = output_path.join(&new_file_name);
        
        // Цикл которой проверяет если по новому пути файл
        while destination_path.exists() {
            // Если нет флага force то ошибка
            if !force {
                return Err("The file already exists in the target directory".to_string());
            }

            // Создание нового имени
            new_file_name = format!("dubbed_{}-{}", counter, &source_file_name.to_string_lossy());
            destination_path = output_path.join(&new_file_name);
            counter += 1;
        }
    
        // Возращение имени
        Ok(new_file_name)
    }

    /// Method for get struct SortingFile
    ///
    /// # Arguments
    ///
    /// * `file_path` - argument path for file
    /// * `output_path` - output dir for file
    /// * `force` - force mode file movement
    ///
    /// # Returns
    /// 
    /// Return struct SortingFile or Err in enum Result<SortingFile, String> 
    pub fn new(file_path: &String, output_path: &String, force: bool) -> Result<SortingFile, String> {
        // Получает расширения
        let extension = Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or(INDEFINITE_FILE_NAME);

        // Создание пути который должен быть в для файла
        
        let destination_dir= output_path.clone()
        + &OPERATING_SYSTEM_SLASH
        + extension;

        // Генерации имени
        let file_name = match Self::current_file_name(
            Path::new(&file_path), 
            Path::new(&destination_dir), 
            force) {
                Ok(name) => name,
                Err(e) => return Err(e)
            };

        // Создание структуры
        Ok(SortingFile {
            file_path: file_path.clone(),
            name: file_name,
            extension: extension.to_string(),
        })
    }
}



pub struct SortingFiles {
    output_path: String,
    grouped_files: Vec<SortingFile>,
}

impl SortingFiles {
    pub fn new(output: &String, files: Vec<SortingFile>) -> SortingFiles {
        SortingFiles {
            output_path: output.clone(),
            grouped_files: files,
        }
    }

    pub fn sort(&self) -> Result<(), String> {
        for item in &self.grouped_files {
            // Получение финальной директории

            let mut destination_dir= self.output_path.clone()
             + &OPERATING_SYSTEM_SLASH
             + &item.extension;

            // Создание директории если ее неты
            let dir = Path::new(&destination_dir);
            if !(dir.exists() && dir.is_dir()) {
                if let Err(e) = fs::create_dir(dir) {
                    let err_message = format!("{}", e);
                    return Err(err_message);
                }
            }
            destination_dir = destination_dir
             + OPERATING_SYSTEM_SLASH
             + &item.name;

            // Перемещение файла
            if let Err(err) = fs::rename(&item.file_path, &destination_dir) {
                let err_message = format!("Error moving file: {}", err);
                return Err(err_message);
            }
        }
        Ok(())
    }
}