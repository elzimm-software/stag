use std::sync::atomic::{AtomicU32, Ordering};

static TRUE_ID: AtomicU32 = AtomicU32::new(0);

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

unsafe fn get_next_id() -> u32 {
    let ptr = NEXT_ID.as_ptr();
    unsafe {
        let id = *ptr;
        *ptr = *ptr + 1;
        id
    }
}

#[derive(Debug, Eq)]
pub struct UniqueBool {
    id: u32,
}

impl PartialEq for UniqueBool {
    fn eq(&self, other: &Self) -> bool {
        !(self.is_true() || other.is_true())
    }
}

impl UniqueBool {
    pub fn new() -> Self {
        Self {
            id: unsafe {get_next_id()},
        }
    }

    pub fn is_true(&self) -> bool {
        self.id == TRUE_ID.load(Ordering::Relaxed)
    }

    pub unsafe fn try_set(&mut self, value: bool) -> Result<(),()> {
        let ptr = TRUE_ID.as_ptr();
        unsafe {
            if *ptr == 0 {
                if value {
                    *ptr = self.id;
                }
                Ok(())
            } else if self.id == *ptr {
                if !value {
                    *ptr = 0;
                }
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialized_false() {
        let a = UniqueBool::new();
        assert!(!a.is_true());
    }

    #[test]
    fn test_set() {
        let mut a = UniqueBool::new();
        unsafe {
            assert_eq!(a.try_set(true), Ok(()));
            assert!(a.is_true());
            assert_eq!(a.try_set(true), Ok(()));
            assert!(a.is_true());
            assert_eq!(a.try_set(false), Ok(()));
            assert!(!a.is_true());
            assert_eq!(a.try_set(false), Ok(()));
            assert!(!a.is_true());
        }
    }

    #[test]
    fn test_set_second_true() {
        let mut a = UniqueBool::new();
        let mut b = UniqueBool::new();
        unsafe {
            let _ = a.try_set(true);
            assert_eq!(b.try_set(true), Err(()));
            assert!(!b.is_true());
            a.try_set(false).unwrap();
            assert_eq!(b.try_set(true), Ok(()));
            assert!(b.is_true());
        }
    }
}