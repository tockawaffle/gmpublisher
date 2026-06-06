use std::path::PathBuf;

pub fn free_caches() {
	crate::game_addons::free_caches();
	crate::steam::workshop::free_caches();
	search!().clear();
}

#[tauri::command]
pub fn check_file(path: PathBuf, extension: Option<String>) -> bool {
	path.is_absolute()
		&& path.is_file()
		&& match extension {
			Some(extension) => {
				if let Some(picked_extension) = path.extension() {
					extension.eq_ignore_ascii_case(&picked_extension.to_string_lossy())
				} else {
					false
				}
			}
			None => true,
		}
}

#[tauri::command]
pub fn check_dir(path: PathBuf) -> bool {
	path.is_absolute() && path.is_dir()
}

#[tauri::command]
pub fn open(path: PathBuf) {
	crate::path::open(path);
}

#[tauri::command]
pub fn open_file_location(path: PathBuf) {
	crate::path::open_file_location(path);
}

#[tauri::command]
pub fn file_size(path: PathBuf) -> Option<u64> {
	path.metadata().ok().map(|metadata| metadata.len())
}
