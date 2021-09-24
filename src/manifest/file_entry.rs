use super::JsValue;
use crate::EnvSubst;

#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct FileEntry(pub JsValue);

impl EnvSubst for FileEntry {
    fn env_subst(
        &self,
        context: &std::collections::HashMap<String, String>,
    ) -> Result<Self, envsubst::Error> {
        Ok(Self(self.0.env_subst(context)?))
    }
}
