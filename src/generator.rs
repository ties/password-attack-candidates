use std::collections::HashSet;

/// Generate all possible variations of a password with the given maximum edit distance
pub fn generate_variations(password: &str, max_distance: u8) -> HashSet<String> {
    let mut variations = HashSet::new();
    
    // Apply distance 1 variations
    let distance_1 = generate_distance_1_variations(password);
    variations.extend(distance_1.clone());
    
    // If max_distance is 2, apply distance 1 variations to the distance 1 variations
    if max_distance >= 2 {
        for var in distance_1 {
            let distance_2 = generate_distance_1_variations(&var);
            variations.extend(distance_2);
        }
    }
    variations
}

/// Generate all possible variations with distance 1
fn generate_distance_1_variations(password: &str) -> HashSet<String> {
    let mut variations = HashSet::new();
    let chars: Vec<char> = password.chars().collect();
    
    // 1. Character substitutions
    for i in 0..chars.len() {
        for c in '!'..'~' {
            if c != chars[i] {
                let mut new_chars = chars.clone();
                new_chars[i] = c;
                variations.insert(new_chars.iter().collect());
            }
        }
    }
    
    // 2. Character deletions
    for i in 0..chars.len() {
        let mut new_chars = chars.clone();
        new_chars.remove(i);
        variations.insert(new_chars.iter().collect());
    }
    
    // 3. Character insertions
    for i in 0..=chars.len() {
        for c in '!'..'~' {
            let mut new_chars = chars.clone();
            new_chars.insert(i, c);
            variations.insert(new_chars.iter().collect());
        }
    }
    
    variations
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance_1_variations() {
        let variations = generate_distance_1_variations("a");
        
        // Check for substitutions
        assert!(variations.contains("b"));
        
        // Check for deletions
        assert!(variations.contains(""));
        
        // Check for insertions
        assert!(variations.contains("aa"));
        assert!(variations.contains("ba"));
        
        // Original should be preserved in the set
        assert!(variations.contains("a"));
    }
    
    #[test]
    fn test_variations_count_simple() {
        let variations = generate_variations("a", 1);
        
        // For "a", we expect:
        // - 93 substitutions (printable ASCII without 'a')
        // - 1 deletion (empty string)
        // - 94 insertions before 'a' (all printable ASCII)
        // - 94 insertions after 'a' (all printable ASCII)
        assert_eq!(variations.len(), 93 + 1 + 94 + 94);
    }
    
    #[test]
    fn test_distance_2_variations() {
        let password = "a";
        // Get all distance <= 2 variations (not including original)
        let variations = generate_variations(password, 2);
        
        // Check for a distance 2 variation: substitute 'a' -> 'b' and then insert 'c'
        assert!(variations.contains("bc") || variations.contains("cb"));
        
        // Should be a lot more variations with distance 2
        assert!(variations.len() > 1000);
    }
}