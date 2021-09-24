use super::*;

impl<T> EnvSubst for Option<T>
where
    T: EnvSubst,
{
    fn env_subst(&self, context: &HashMap<String, String>) -> Result<Self, SubstError> {
        self.as_ref().map(|v| v.env_subst(context)).transpose()
    }
}
