use std::path::PathBuf;
use std::fs;


enum SizeOption {

}

enum Mode {

}


fn get_image_files(path:PathBuf)-> Result<Vec<PathBuf>,ImagixError>{

    let entries = fs::read_dir(path)
                  .map_err(|e| ImagixError::UserInputError("Invalid
                  source folder".to_string()))?
                  .map(|res| res.map(|e| e.path()))
                  .collect::<Result<Vec<_>, io::Error>>()?
                  .into_iter()
                  .filter(|r| {
                  r.extension() == Some("JPG".as_ref())
                  || r.extension() == Some("jpg".as_ref())
                  || r.extension() == Some("PNG".as_ref())
                  || r.extension() == Some("png".as_ref())
                  })
                  .collect();
                  Ok(entries)
                 }


}