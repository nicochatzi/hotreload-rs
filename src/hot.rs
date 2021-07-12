use crate::{err::HotreloadResult, file};
use hotwatch::Hotwatch;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

#[derive(Debug)]
struct SharedLibrary {
    handle: Option<lib::Library>,
    origin: PathBuf,
    loaded: PathBuf,
}

impl SharedLibrary {
    fn from_path(path: impl AsRef<Path>) -> Self {
        let duplicate = file::duplicate(path.as_ref()).unwrap();
        Self {
            handle: Some(unsafe { lib::Library::new(duplicate.as_os_str()).unwrap() }),
            origin: path.as_ref().to_path_buf(),
            loaded: duplicate,
        }
    }

    fn reload(&mut self) {
        self.handle.take().unwrap().close().unwrap();
        fs::remove_file(&self.loaded).expect("Failed to remove dylib");
        *self = SharedLibrary::from_path(&self.origin);
    }
}

#[derive(Debug)]
pub struct HotLibrary {
    shared: Arc<RwLock<SharedLibrary>>,
    _watcher: Hotwatch,
}

impl HotLibrary {
    pub fn new(folder_relative_path: &str, lib_name: &str) -> HotreloadResult<Self> {
        let lib_path = file::dll(folder_relative_path, lib_name)?;
        let lib = Arc::new(RwLock::new(SharedLibrary::from_path(&lib_path)));
        let lib_clone = lib.clone();
        let mut watcher = Hotwatch::new()?;
        watcher.watch(lib_path, move |_| lib_clone.write().unwrap().reload())?;
        Ok(Self {
            shared: lib,
            _watcher: watcher,
        })
    }

    pub fn call_or_fallback<Signature, LibFn, FallFn, Ret>(
        &self,
        symbol_name: &str,
        mut func: LibFn,
        mut fallback: FallFn,
    ) -> Ret
    where
        LibFn: FnMut(lib::Symbol<'_, Signature>) -> Ret,
        FallFn: FnMut() -> Ret,
    {
        let mut try_call = || {
            let shared = self.shared.read().ok()?;
            let symbol = unsafe { shared.handle.as_ref()?.get(symbol_name.as_bytes()).ok()? };
            Some(func(symbol))
        };
        match try_call() {
            Some(ret) => ret,
            None => fallback(),
        }
    }
}
