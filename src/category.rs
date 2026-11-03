pub const CAT_A_PLUS_SPEED: f64 = 45.0;
pub const CAT_A_SPEED: f64 = 42.0;
pub const CAT_B_SPEED: f64 = 37.0;
pub const CAT_C_SPEED: f64 = 33.0;
pub const CAT_D_SPEED: f64 = 30.9;
pub const CAT_E_SPEED: f64 = 28.0;

pub fn get_category_from_score(zwift_score: u32) -> &'static str {
    match zwift_score {
        0..=99 => "E",
        100..=199 => "D",
        200..=299 => "C",
        300..=399 => "B",
        400..=599 => "A",
        600.. => "A++",
    }
}

pub fn get_detailed_category_from_score(zwift_score: u32) -> &'static str {
    match zwift_score {
        0..=49 => "E-",
        50..=99 => "E",
        100..=149 => "D-",
        150..=189 => "D",
        190..=199 => "D+",
        200..=249 => "C-",
        250..=289 => "C",
        290..=299 => "C+",
        300..=349 => "B-",
        350..=389 => "B",
        390..=399 => "B+",
        400..=499 => "A-",
        500..=589 => "A",
        590..=649 => "A+",
        650.. => "A++",
    }
}

pub fn get_category_speed(category: &str) -> f64 {
    match category {
        "A++" => CAT_A_PLUS_SPEED,
        "A" | "A+" | "A-" => CAT_A_SPEED,
        "B" | "B+" | "B-" => CAT_B_SPEED,
        "C" | "C+" | "C-" => CAT_C_SPEED,
        "E" | "E+" | "E-" => CAT_E_SPEED,
        _ => CAT_D_SPEED,
    }
}

pub fn category_matches_subgroup(user_category: &str, subgroup_name: &str) -> bool {
    if subgroup_name.contains(user_category) {
        return true;
    }

    if user_category == "D" && subgroup_name.contains("E") {
        return true;
    }

    if user_category.len() > 1 {
        let base_category = &user_category[0..1];
        if subgroup_name.contains(&format!(" {}", base_category))
            || subgroup_name.contains(&format!("{} ", base_category))
            || subgroup_name.contains(&format!("({})", base_category))
            || subgroup_name.contains(&format!("Category {}", base_category))
            || subgroup_name.contains(&format!("Cat {}", base_category))
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_category_from_score() {
        assert_eq!(get_category_from_score(0), "E");
        assert_eq!(get_category_from_score(50), "E");
        assert_eq!(get_category_from_score(99), "E");

        assert_eq!(get_category_from_score(100), "D");
        assert_eq!(get_category_from_score(150), "D");
        assert_eq!(get_category_from_score(199), "D");

        assert_eq!(get_category_from_score(200), "C");
        assert_eq!(get_category_from_score(250), "C");
        assert_eq!(get_category_from_score(299), "C");

        assert_eq!(get_category_from_score(300), "B");
        assert_eq!(get_category_from_score(350), "B");
        assert_eq!(get_category_from_score(399), "B");

        assert_eq!(get_category_from_score(400), "A");
        assert_eq!(get_category_from_score(500), "A");
        assert_eq!(get_category_from_score(599), "A");

        assert_eq!(get_category_from_score(600), "A++");
        assert_eq!(get_category_from_score(700), "A++");
        assert_eq!(get_category_from_score(999), "A++");
    }

    #[test]
    fn test_get_detailed_category_from_score() {
        assert_eq!(get_detailed_category_from_score(25), "E-");
        assert_eq!(get_detailed_category_from_score(75), "E");

        assert_eq!(get_detailed_category_from_score(125), "D-");
        assert_eq!(get_detailed_category_from_score(175), "D");
        assert_eq!(get_detailed_category_from_score(195), "D+");

        assert_eq!(get_detailed_category_from_score(225), "C-");
        assert_eq!(get_detailed_category_from_score(275), "C");
        assert_eq!(get_detailed_category_from_score(295), "C+");

        assert_eq!(get_detailed_category_from_score(325), "B-");
        assert_eq!(get_detailed_category_from_score(375), "B");
        assert_eq!(get_detailed_category_from_score(395), "B+");

        assert_eq!(get_detailed_category_from_score(450), "A-");
        assert_eq!(get_detailed_category_from_score(550), "A");
        assert_eq!(get_detailed_category_from_score(650), "A++");
    }

    #[test]
    fn test_get_category_speed() {
        assert_eq!(get_category_speed("A++"), 45.0);
        assert_eq!(get_category_speed("A+"), 42.0);
        assert_eq!(get_category_speed("A"), 42.0);
        assert_eq!(get_category_speed("B"), 37.0);
        assert_eq!(get_category_speed("C"), 33.0);
        assert_eq!(get_category_speed("D"), 30.9);
        assert_eq!(get_category_speed("E"), 28.0);

        assert_eq!(get_category_speed("D+"), 30.9);
        assert_eq!(get_category_speed("C-"), 33.0);
        assert_eq!(get_category_speed("B+"), 37.0);
        assert_eq!(get_category_speed("A-"), 42.0);

        assert_eq!(get_category_speed("X"), 30.9);
        assert_eq!(get_category_speed(""), 30.9);
    }

    #[test]
    fn test_category_matches_subgroup() {
        assert!(category_matches_subgroup("A", "Category A"));
        assert!(category_matches_subgroup("B", "Cat B Men"));
        assert!(category_matches_subgroup("C", "C - Women"));

        assert!(category_matches_subgroup("D", "Category E"));
        assert!(category_matches_subgroup("D", "Cat D/E Mixed"));

        assert!(category_matches_subgroup("D+", "Category D"));
        assert!(category_matches_subgroup("C-", "Cat C"));

        assert!(!category_matches_subgroup("C", "B - Men's Race"));
        assert!(!category_matches_subgroup("A", "Cat D"));
        assert!(!category_matches_subgroup("B", "A Elite"));
    }
}
