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

pub fn generate_variations(password: &str, max_distance: usize, transposition_distance: usize) -> Vec<String> {
    let mut all_variations = HashSet::new();
    all_variations.insert(password.to_string());
    
    // Track all variations at each distance level to build up next level
    let mut variations_by_distance: Vec<HashSet<String>> = Vec::with_capacity(max_distance);
    variations_by_distance.push(vec![password.to_string()].into_iter().collect());
    
    // Use a loop to build up variations for each distance level up to max_distance
    for distance in 1..=max_distance {
        let mut next_distance = HashSet::new();
        
        // Apply distance 1 variations to all variations from the previous distance
        for var in &variations_by_distance[distance - 1] {
            let new_variations = generate_distance_1_variations(var);
            next_distance.extend(new_variations);
        }
        
        variations_by_distance.push(next_distance.clone());
        all_variations.extend(next_distance);
    }
    
    // Add transposition variations (for each distance level)
    for distance in 1..=transposition_distance {
        // Only consider reasonable transposition offsets
        let max_offset = distance.min(3); // Limit to maximum offset of 3
        let base_words = all_variations.clone();
        
        for offset in 1..=max_offset {
            for word in &base_words {
                if offset <word.len() {
                    let transpositions = generate_transposition_variations(word, offset);
                    all_variations.extend(transpositions);
                }
            }
        }
    }
    
    all_variations.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance_1_variations() {
        let variations = generate_distance_1_variations("a");
        
        // Check for substitutions
        assert!(variations.contains(&"b".to_string()));
        
        // Check for deletions
        assert!(variations.contains(&"".to_string()));
        
        // Check for insertions
        assert!(variations.contains(&"aa".to_string()));
        assert!(variations.contains(&"ba".to_string()));
        
        // Original should be preserved in the set
        assert!(variations.contains(&"a".to_string()));
    }
    
    #[test]
    fn test_variations_count_simple() {
        let variations = generate_variations("a", 1, 1);
        
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
        let variations = generate_variations(password, 2, 1);
        
        // Check for a distance 2 variation: substitute 'a' -> 'b' and then insert 'c'
        assert!(variations.contains(&"bc".to_string()) || variations.contains(&"cb".to_string()));
        
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
        let variations = generate_variations("abc", 1, 1);
        // Should include transpositions with offset=1
        assert!(variations.contains(&"bac".to_string()));
        assert!(variations.contains(&"acb".to_string()));
    }
    
    #[test]
    fn test_transposition_distance_parameter() {
        // With transposition_distance = 0, no transpositions should be included
        let variations_no_trans = generate_variations("abcde", 1, 0);
        assert!(!variations_no_trans.contains(&"bacde".to_string())); // No adjacent swap
        
        // With transposition_distance = 1, only offset=1 transpositions
        let variations_trans_1 = generate_variations("abcde", 1, 1);
        assert!(variations_trans_1.contains(&"bacde".to_string())); // Adjacent swap
        assert!(!variations_trans_1.contains(&"cbade".to_string())); // Offset=2 swap not included
        
        // With transposition_distance = 2, both offset=1 and offset=2 transpositions
        let variations_trans_2 = generate_variations("abcde", 1, 2);
        assert!(variations_trans_2.contains(&"bacde".to_string())); // Adjacent swap
        assert!(variations_trans_2.contains(&"cbade".to_string())); // Offset=2 swap included
        assert!(variations_trans_2.contains(&"abedc".to_string())); // Offset=2 swap at different position
        
        // With transposition_distance = 3, offset=3 transpositions should be included
        let variations_trans_3 = generate_variations("abcde", 1, 3);
        assert!(variations_trans_3.contains(&"dbcae".to_string())); // Offset=3 swap
    }
}