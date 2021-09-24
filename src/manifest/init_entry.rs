use super::JsValue;
use crate::EnvSubst;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct InitEntry(pub JsValue);

impl EnvSubst for InitEntry {
    fn env_subst(
        &self,
        context: &std::collections::HashMap<String, String>,
    ) -> Result<Self, envsubst::Error> {
        Ok(Self(self.0.env_subst(context)?))
    }
}
