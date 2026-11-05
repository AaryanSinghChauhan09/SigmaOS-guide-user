pub struct UserGuideBook;
impl UserGuideBook {
    pub fn get_handbook_title() -> &'static str { "SigmaOS User Guide & Workflow Handbook" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_guide_title() {
        assert_eq!(UserGuideBook::get_handbook_title(), "SigmaOS User Guide & Workflow Handbook");
    }
}
