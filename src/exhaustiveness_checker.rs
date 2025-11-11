use crate::ast::*;

/// Exhaustiveness checker for pattern matching
pub struct ExhaustivenessChecker;

impl ExhaustivenessChecker {
    /// Check if a match expression is exhaustive
    pub fn check_exhaustive(_expr: &Expr, arms: &[MatchArm]) -> Result<(), String> {
        // Simplified exhaustiveness checking
        // In a full implementation, this would:
        // 1. Analyze the type of the matched expression
        // 2. Check if all possible values are covered
        // 3. Handle nested patterns
        // 4. Handle wildcards and guards
        
        // For now, just check if there's a wildcard pattern
        let has_wildcard = arms.iter().any(|arm| {
            matches!(arm.pattern, Pattern::Wildcard)
        });
        
        if !has_wildcard {
            // Check if all enum variants are covered (simplified)
            // This would need type information in a full implementation
        }
        
        Ok(())
    }
    
    /// Check if patterns overlap (should be an error)
    pub fn check_overlap(arms: &[MatchArm]) -> Result<(), String> {
        // Check for overlapping patterns
        for (i, arm1) in arms.iter().enumerate() {
            for arm2 in arms.iter().skip(i + 1) {
                if Self::patterns_overlap(&arm1.pattern, &arm2.pattern) {
                    return Err(format!("Overlapping patterns detected"));
                }
            }
        }
        Ok(())
    }
    
    fn patterns_overlap(p1: &Pattern, p2: &Pattern) -> bool {
        match (p1, p2) {
            (Pattern::Wildcard, _) | (_, Pattern::Wildcard) => true,
            (Pattern::Number(n1), Pattern::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (Pattern::String(s1), Pattern::String(s2)) => s1 == s2,
            (Pattern::Boolean(b1), Pattern::Boolean(b2)) => b1 == b2,
            (Pattern::Tuple(pats1), Pattern::Tuple(pats2)) => {
                if pats1.len() == pats2.len() {
                    pats1.iter().zip(pats2.iter())
                        .any(|(p1, p2)| Self::patterns_overlap(p1, p2))
                } else {
                    false
                }
            },
            (Pattern::List(pats1), Pattern::List(pats2)) => {
                if pats1.len() == pats2.len() {
                    pats1.iter().zip(pats2.iter())
                        .any(|(p1, p2)| Self::patterns_overlap(p1, p2))
                } else {
                    false
                }
            },
            (Pattern::Or(pats1), Pattern::Or(pats2)) => {
                pats1.iter().any(|p1| {
                    pats2.iter().any(|p2| Self::patterns_overlap(p1, p2))
                })
            },
            _ => false,
        }
    }
    
    /// Get uncovered patterns (for exhaustiveness warnings)
    pub fn get_uncovered_patterns(_expr: &Expr, _arms: &[MatchArm]) -> Vec<String> {
        // Simplified - would need type information
        vec![]
    }
}

