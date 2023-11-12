use godot::prelude::*;

pub mod assets;
pub mod dude;
pub mod fs;

struct GdFalloutExt;

#[gdextension]
unsafe impl ExtensionLibrary for GdFalloutExt {}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
