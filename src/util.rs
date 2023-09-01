use crate::singleton::SingletonClass;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};

pub fn get_instance<T: unity::prelude::Il2CppClassData>() -> &'static mut Il2CppObject<T> {
    let idk = get_generic_class!(SingletonClass<T>).unwrap();
    let pointer = unsafe { &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
    let get_instance = unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut Il2CppObject<T>>(pointer[5].method_ptr) };

    get_instance(Some(&pointer[5]))
}
