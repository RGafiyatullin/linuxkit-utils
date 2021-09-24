use super::*;

use ::serde_yaml::Value as JsValue;

impl EnvSubst for JsValue {
    fn env_subst(&self, context: &HashMap<String, String>) -> Result<Self, SubstError> {
        match self {
            JsValue::Bool(_) => Ok(self.to_owned()),
            JsValue::Null => Ok(self.to_owned()),
            JsValue::Number(_) => Ok(self.to_owned()),
            JsValue::String(s) => Ok(JsValue::String(s.env_subst(context)?)),
            JsValue::Sequence(items) => Ok(JsValue::Sequence(
                items
                    .iter()
                    .map(|item| item.env_subst(context))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            JsValue::Mapping(mapping) => {
                let mapping = mapping
                    .iter()
                    .map(|(k, v)| {
                        k.env_subst(context)
                            .and_then(|k| v.env_subst(context).map(move |v| (k, v)))
                    })
                    .collect::<Result<_, _>>()?;
                Ok(JsValue::Mapping(mapping))
            }
        }
    }
}
