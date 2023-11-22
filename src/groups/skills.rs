#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Skills
pub struct SkillsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct CharacterSkills {
    pub skills: Vec<Skill>,
    pub total_sp: i64,
    pub unallocated_sp: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs)]
pub struct Skill {
    pub active_skill_level: i32,
    pub skill_id: i32,
    pub skillpoints_in_skill: i64,
    pub trained_skill_level: i32,
}

impl<'a> SkillsGroup<'a> {
    api_get!(
        /// List all trained skills for the given character
        get_character_skills,
        "get_characters_character_id_skills",
        RequestType::Authenticated,
        CharacterSkills,
        (character_id: i32) => "{character_id}"
    );
}
