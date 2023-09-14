use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SettingsMap {
    map: HashMap<String, AnyValue>,
}

impl SettingsMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: std::any::Any + Send + Clone + Sync + 'static>(
        &mut self,
        key: String,
        data: T,
    ) {
        self.map.insert(key, AnyValue::new(data));
    }

    pub fn insert_raw(&mut self, key: String, data: AnyValue) {
        self.map.insert(key, data);
    }

    pub fn get<T: std::any::Any + Send + Clone + Sync + 'static>(&self, key: &str) -> Option<&T> {
        let Some(data) = self.map.get(key) else {
            return None;
        };
        data.downcast_ref()
    }
}

#[derive(Debug)]
pub struct AnyValue {
    data: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
}

impl AnyValue {
    pub(crate) fn new<V: std::any::Any + Clone + Send + Sync + 'static>(data: V) -> Self {
        Self {
            data: std::sync::Arc::new(data),
        }
    }
    pub(crate) fn downcast_ref<T: std::any::Any + Clone + Send + Sync + 'static>(
        &self,
    ) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    // pub(crate) fn downcast_into<T: std::any::Any + Clone + Send + Sync>(self) -> Result<T, Self> {
    //     let value = std::sync::Arc::downcast::<T>(self.data).map_err(|data| Self { data })?;
    //     let value = std::sync::Arc::try_unwrap(value).unwrap_or_else(|arc| (*arc).clone());
    //     Ok(value)
    // }
}
