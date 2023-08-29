use crate::*;

pub fn get_memory(key: impl AsRef<str>) -> Result<Option<Memory>, Error> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes())?;

    let offset = unsafe { extism_var_get(mem.offset()) };
    if offset == 0 {
        return Ok(None);
    }
    let length = unsafe { extism_length(offset) };

    if length == 0 {
        return Ok(None);
    }

    let memory = MemoryHandle { offset, length };
    Ok(Some(Memory(memory)))
}

pub fn get<T: FromBytesOwned>(key: impl AsRef<str>) -> Result<Option<T>, Error> {
    match get_memory(key)?.map(|x| x.to_vec()) {
        Some(v) => Ok(Some(T::from_bytes(&v)?)),
        None => Ok(None),
    }
}

pub fn set(key: impl AsRef<str>, val: impl ToMemory) -> Result<(), Error> {
    let val = val.to_memory()?;
    let key = Memory::from_bytes(key.as_ref().as_bytes())?;
    unsafe { extism_var_set(key.offset(), val.offset()) }
    Ok(())
}

pub fn remove(key: impl AsRef<str>) -> Result<(), Error> {
    let key = Memory::from_bytes(key.as_ref().as_bytes())?;
    unsafe { extism_var_set(key.offset(), 0) };
    Ok(())
}
