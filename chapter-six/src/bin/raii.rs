use std::ops::Deref;


struct SomeOsSpecificFunctionalityHandle;

struct SomeOsFunctionality<T> {
    data: T,
    inner: Box<SomeOsSpecificFunctionalityHandle>,
}

struct SomeOsFunctionalityGuard<'a, T: 'a>{
    lock: &'a SomeOsFunctionality<T>,
}

impl SomeOsSpecificFunctionalityHandle {
    unsafe fn lock(&self) {

    }

    unsafe fn unlock(&self){

    }
}

impl<T> SomeOsFunctionality<T>{
    fn new(data: T) -> Self {
        let handle = SomeOsSpecificFunctionalityHandle;
        SomeOsFunctionality {
            data,
            inner: Box::new(handle),
        }
    }

    fn lock(&self) -> SomeOsFunctionalityGuard<T> {
        unsafe {
            self.inner.lock();
        }

        SomeOsFunctionalityGuard { lock: self }
    }
}

impl<'a, T> Drop for SomeOsFunctionalityGuard<'a, T> {
    fn drop(&mut self){
        unsafe {
            self.lock.inner.unlock();
        }
    }
}

impl<'a, T> Deref for SomeOsFunctionalityGuard<'a, T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.lock.data
    }
}

fn main() {
    let foo = SomeOsFunctionality::new("Hello World");
    {
        let bar = foo.lock();
        println!("The string behind foo is {} characters long", bar.len());
    }
}