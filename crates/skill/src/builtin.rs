pub struct BuiltinSkill {
    pub name: &'static str,
    pub content: &'static str,
}

pub const BUILTIN_SKILLS: &[BuiltinSkill] = &[
    BuiltinSkill {
        name: "task-manager",
        content: include_str!("../../../workspace/skills/task-manager/SKILL.md"),
    },
    BuiltinSkill {
        name: "knowledge-base",
        content: include_str!("../../../workspace/skills/knowledge-base/SKILL.md"),
    },
];

pub fn find_builtin(name: &str) -> Option<&'static BuiltinSkill> {
    let clean = name.trim().to_lowercase();
    BUILTIN_SKILLS.iter().find(|s| s.name == clean)
}

pub fn all_builtins() -> &'static [BuiltinSkill] {
    BUILTIN_SKILLS
}
