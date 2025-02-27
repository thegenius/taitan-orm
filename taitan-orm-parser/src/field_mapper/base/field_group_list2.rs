// 6个类型
// (1) LeadingRequired，第一组的required字段，前面没有Option
// (2) LeadingFailRequired，第一组required字段，但是前面有option字段
// (3) TrailingRequired，非第一组的required字段
// (4) LeadingOptional，第一个optional字段，且前面没有required字段
// (5) FollowingOptional，非第一个optional字段，且前面没有required字段
// (6) TrailingOptional，optional字段，无论是否是第一个，只有前面有Required字段，就是TrailingOptional
//
// ___________________________________________________________

use crate::field_mapper::base::LeadingCommaType;
use crate::FieldDef;

pub enum FieldGroup<'a> {
    LeadingRequired {
        fields: Vec<FieldDef<'a>>,
        comma_type: LeadingCommaType,
    },
    LeadingFailRequired {
        fields: Vec<FieldDef<'a>>,
        comma_type: LeadingCommaType,
    },
    TrailingRequired {
        fields: Vec<FieldDef<'a>>,
        comma_type: LeadingCommaType,
    },
    LeadingOptional {
        field: FieldDef<'a>,
        comma_type: LeadingCommaType,
    },
    FollowingOptional {
        field: FieldDef<'a>,
        comma_type: LeadingCommaType,
    },
    TrailingOptional {
        field: FieldDef<'a>,
        comma_type: LeadingCommaType,
    },
}

impl<'a> FieldGroup<'a> {
    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            FieldGroup::LeadingOptional { .. }
                | FieldGroup::FollowingOptional { .. }
                | FieldGroup::TrailingOptional { .. }
        )
    }

    pub fn len(&self) -> usize {
        match self {
            Self::LeadingRequired { fields, .. }
            | Self::LeadingFailRequired { fields, .. }
            | Self::TrailingRequired { fields, .. } => fields.len(),
            _ => 1,
        }
    }
}

pub struct FieldGroupList<'a> {
    pub groups: Vec<FieldGroup<'a>>,
    pub first_required: usize,
    pub is_all_required: bool,
}

impl<'a> FieldGroupList<'a> {
    pub fn from<T>(fields: T) -> Self
    where
        T: IntoIterator<Item=&'a FieldDef<'a>> + Clone,
    {
        let mut groups = Vec::new();
        let mut current_group = Vec::new();
        let mut has_required = false; // 是否已经出现过 required 字段
        let mut has_optional = false; // 是否已经出现过 optional 字段
        let mut is_first_required_group = true; // 是否是第一组 required 字段

        for field in fields.clone().into_iter() {
            if field.struct_field.is_optional {
                // 如果当前组不为空，先将其加入结果
                if !current_group.is_empty() {
                    if !has_required {
                        // 如果前面没有 required 字段，则是 LeadingRequired
                        groups.push(FieldGroup::LeadingRequired {
                            fields: current_group,
                            comma_type: LeadingCommaType::NoLeading,
                        });
                        has_required = true;
                    } else if is_first_required_group {
                        // 如果前面有 optional 字段，且是第一组 required，则是 LeadingFailRequired
                        groups.push(FieldGroup::LeadingFailRequired {
                            fields: current_group,
                            comma_type: LeadingCommaType::CheckedLeading,
                        });
                        is_first_required_group = false;
                    } else {
                        // 否则是 TrailingRequired
                        groups.push(FieldGroup::TrailingRequired {
                            fields: current_group,
                            comma_type: LeadingCommaType::Leading,
                        });
                    }
                    current_group = Vec::new();
                }

                // 处理 optional 字段
                if !has_required && !has_optional {
                    // 如果前面没有 required 和 optional 字段，则是 LeadingOptional
                    groups.push(FieldGroup::LeadingOptional {
                        field: field.clone(),
                        comma_type: LeadingCommaType::NoLeading,
                    });
                } else if !has_required {
                    // 如果前面没有 required 字段，但有 optional 字段，则是 FollowingOptional
                    groups.push(FieldGroup::FollowingOptional {
                        field: field.clone(),
                        comma_type: LeadingCommaType::CheckedLeading,
                    });
                } else {
                    // 如果前面有 required 字段，则是 TrailingOptional
                    groups.push(FieldGroup::TrailingOptional {
                        field: field.clone(),
                        comma_type: LeadingCommaType::Leading,
                    });
                }
                has_optional = true;
            } else {
                // 将 is_optional 为 false 的字段加入当前组
                current_group.push(field.clone());
            }
        }

        // 处理最后一组
        if !current_group.is_empty() {
            if !has_required {
                // 如果前面没有 required 字段，则是 LeadingRequired
                groups.push(FieldGroup::LeadingRequired {
                    fields: current_group,
                    comma_type: LeadingCommaType::NoLeading,
                });
            } else if is_first_required_group {
                // 如果前面有 optional 字段，且是第一组 required，则是 LeadingFailRequired
                groups.push(FieldGroup::LeadingFailRequired {
                    fields: current_group,
                    comma_type: LeadingCommaType::CheckedLeading,
                });
            } else {
                // 否则是 TrailingRequired
                groups.push(FieldGroup::TrailingRequired {
                    fields: current_group,
                    comma_type: LeadingCommaType::Leading,
                });
            }
        }

        let is_all_required = fields.into_iter().all(|f| !f.struct_field.is_optional);

        let first_required = {
            let mut first_required_index = groups.len();
            for (index, group) in groups.iter().enumerate() {
                if matches!(
                    group,
                    FieldGroup::LeadingRequired { .. }
                        | FieldGroup::LeadingFailRequired{..}
                        | FieldGroup::TrailingRequired{..}
                ) {
                    first_required_index = index;
                    break;
                }
            }
            first_required_index
        };

        Self {
            groups,
            first_required,
            is_all_required,
        }
    }
}
