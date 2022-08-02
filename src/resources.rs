use std::any::Any;
use std::any::TypeId;

pub trait Resource: Any + Send + Sync {}
impl<T> Resource for T where T: Any + Send + Sync {}

#[derive(Default)]
pub struct ResourceManager {
    values: indexmap::IndexMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceManager {
    pub fn insert<R: Resource>(&mut self, resource: R) -> usize {
        let (id, _) = self.values.insert_full(TypeId::of::<R>(), Box::new(resource));
        id
    }

    #[inline]
    pub fn fetch<R: Resource>(&mut self) -> &R {
        let item = self.values.get(&TypeId::of::<R>()).unwrap();
        item.downcast_ref::<R>().unwrap()
    }

    #[inline]
    pub fn fetch_mut<R: Resource>(&mut self) -> &mut R {
        let item = self.values.get(&TypeId::of::<R>()).unwrap();
        item.downcast_mut::<R>().unwrap()
    }

    #[inline]
    pub fn get<R: Resource>(&mut self) -> Option<&R> {
        let item = self.values.get(&TypeId::of::<R>()).unwrap();
        unsafe { item.downcast_ref::<R>() }
    }

    #[inline]
    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        let item = self.values.get_mut(&TypeId::of::<R>()).unwrap();
        unsafe { item.downcast_mut::<R>() }
    }
}
