use std::sync::Mutex;

static TRUE_ID: Mutex<u32> = Mutex::new(0);

static NEXT_ID: Mutex<u32> = Mutex::new(1);

fn get_next_id() -> u32 {
    let mut ptr = NEXT_ID.lock().expect("Error: NEXT_ID is poisoned");
    let id = *ptr;
    *ptr = *ptr + 1;
    id
}

pub struct UniqueBool {
    id: u32,
}

impl UniqueBool {
    pub fn new() -> UniqueBool {
        Self {
            id: get_next_id(),
        }
    }

    pub fn is_true(&self) -> bool {
        self.id == *TRUE_ID.lock().expect("Error: TRUE_ID is poisoned.")
    }

    pub fn try_set(&mut self, value: bool) -> Result<(),()> {
        let mut ptr = TRUE_ID.lock().expect("Error: TRUE_ID is poisoned.");
        if *ptr == 0 && value {
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
        assert_eq!(a.try_set(true), Ok(()));
        assert!(a.is_true());
        assert_eq!(a.try_set(true), Ok(()));
        assert!(a.is_true());
        assert_eq!(a.try_set(false), Ok(()));
        assert!(!a.is_true());
    }

    #[test]
    fn test_set_second_true() {
        let mut a = UniqueBool::new();
        let mut b = UniqueBool::new();
        a.try_set(true).unwrap();
        assert_eq!(b.try_set(true), Err(()));
        assert!(!b.is_true());
        a.try_set(false).unwrap();
        assert_eq!(b.try_set(true), Ok(()));
        assert!(b.is_true());
    }
}