use std::collections::HashSet;

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

/// Generates password variations with character transpositions at the given offset.
/// 
/// # Arguments
/// 
/// * `password` - The original password string
/// * `offset` - How far to shift characters (1 means swap with adjacent character)
/// 
/// # Examples
/// 
/// ```
/// use generator::generate_transposition_variations;
/// 
/// let variations = generate_transposition_variations("abc", 1);
/// assert!(variations.contains(&"bac".to_string()));
/// assert!(variations.contains(&"acb".to_string()));
/// ```
pub fn generate_transposition_variations(password: &str, offset: usize) -> Vec<String> {
    let mut variations = Vec::new();
    let chars: Vec<char> = password.chars().collect();
    
    // Skip if password is too short for the given offset
    if chars.len() <= offset {
        return variations;
    }
    
    // Generate transpositions
    for i in 0..(chars.len() - offset) {
        let mut transposed = chars.clone();
        // Swap character with the one at offset distance
        transposed.swap(i, i + offset);
        
        variations.push(transposed.into_iter().collect());
    }
    
    variations
}

pub fn generate_variations(password: &str, max_distance: usize) -> Vec<String> {
    let mut all_variations = Vec::new();
    
    // Apply distance 1 variations
    let distance_1 = generate_distance_1_variations(password);
    all_variations.extend(distance_1.clone());
    
    // If max_distance is 2, apply distance 1 variations to the distance 1 variations
    if max_distance >= 2 {
        for var in distance_1 {
            let distance_2 = generate_distance_1_variations(&var);
            all_variations.extend(distance_2);
        }
    }
    
    // Add transposition variations (for each distance level)
    for distance in 1..=max_distance {
        // Only consider reasonable transposition offsets
        let max_offset = distance.min(3); // Limit to maximum offset of 3
        
        for offset in 1..=max_offset {
            if offset < password.len() {
                let transpositions = generate_transposition_variations(password, offset);
                all_variations.extend(transpositions);
            }
        }
    }
    
    all_variations
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

    #[test]
    fn test_transposition_variations() {
        // Test adjacent character swap (offset = 1)
        let variations = generate_transposition_variations("password", 1);
        assert!(variations.contains(&"apssword".to_string())); // 'p' and 'a' swapped
        assert!(variations.contains(&"psasword".to_string())); // 's' and 'a' swapped
        assert!(variations.contains(&"passwrod".to_string())); // 'o' and 'd' swapped
        assert_eq!(variations.len(), 7); // 8-character password has 7 possible adjacent swaps
        
        // Test offset = 2
        let variations = generate_transposition_variations("abc", 2);
        assert!(variations.contains(&"cba".to_string())); // 'a' and 'c' swapped
        assert_eq!(variations.len(), 1); // Only one possible transposition with offset=2 in 3-character string
        
        // Test with password shorter than offset
        let variations = generate_transposition_variations("ab", 2);
        assert_eq!(variations.len(), 0); // No variations possible
    }
    
    #[test]
    fn test_transpositions_included_in_variations() {
        let variations = generate_variations("abc", 1);
        // Should include transpositions with offset=1
        assert!(variations.contains(&"bac".to_string()));
        assert!(variations.contains(&"acb".to_string()));
    }
}