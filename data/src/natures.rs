pub use crate::generated::Nature;
use crate::{Language, NamesData};

impl Nature {
    pub fn name(self, language: Language) -> &'static str {
        self.data().names.get(language)
    }
}

pub(crate) struct NatureData {
    pub(crate) names: NamesData,
}
