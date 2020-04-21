use serenity::model::guild::PartialMember;

pub(crate) fn is_dm(pm: &PartialMember) -> bool {
  pm.roles
    .iter()
    .any(|&i| *i.as_u64() == *crate::DM_ROLE_ID.as_u64())
}
