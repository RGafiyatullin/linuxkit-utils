use super::*;

impl<T> EnvSubst for Vec<T>
where
    T: EnvSubst,
{
    fn env_subst(&self, context: &HashMap<String, String>) -> Result<Self, SubstError> {
        self.iter().map(|item| item.env_subst(context)).collect()
    }
}
