use cel_interpreter::{CelContext, CelMap, CelValue};

pub struct TxParams {
    values: CelMap,
}

impl TxParams {
    pub fn new() -> Self {
        Self {
            values: CelMap::new(),
        }
    }

    pub fn insert(&mut self, k: impl Into<String>, v: impl Into<CelValue>) {
        self.values.insert(k.into(), v.into());
    }
}

impl From<TxParams> for CelContext {
    fn from(p: TxParams) -> Self {
        let mut ctx = CelContext::new();
        ctx.add_variable("params", p.values);
        ctx
    }
}
