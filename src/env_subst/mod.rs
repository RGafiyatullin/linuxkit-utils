use ::envsubst::Error as SubstError;
use std::collections::HashMap;

mod for_jsvalue;
mod for_option;
mod for_string;
mod for_vec;

pub trait EnvSubst: Sized {
    fn env_subst(&self, context: &HashMap<String, String>) -> Result<Self, SubstError>;
}
