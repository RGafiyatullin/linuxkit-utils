use super::*;

impl EnvSubst for String {
    fn env_subst(&self, context: &HashMap<String, String>) -> Result<Self, SubstError> {
        ::envsubst::substitute(self, context)
    }
}
