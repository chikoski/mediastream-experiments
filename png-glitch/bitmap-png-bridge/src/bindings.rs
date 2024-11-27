#[allow(dead_code)]
pub mod chikoski {
    #[allow(dead_code)]
    pub mod glitch_art {
        #[allow(dead_code, clippy::all)]
        pub mod png_glitchable {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum FilterType {
                None,
                Sub,
                Up,
                Average,
                Paeth,
            }
            impl ::core::fmt::Debug for FilterType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        FilterType::None => f.debug_tuple("FilterType::None").finish(),
                        FilterType::Sub => f.debug_tuple("FilterType::Sub").finish(),
                        FilterType::Up => f.debug_tuple("FilterType::Up").finish(),
                        FilterType::Average => {
                            f.debug_tuple("FilterType::Average").finish()
                        }
                        FilterType::Paeth => f.debug_tuple("FilterType::Paeth").finish(),
                    }
                }
            }
            impl FilterType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> FilterType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => FilterType::None,
                        1 => FilterType::Sub,
                        2 => FilterType::Up,
                        3 => FilterType::Average,
                        4 => FilterType::Paeth,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ScanLine {
                handle: _rt::Resource<ScanLine>,
            }
            impl ScanLine {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ScanLine {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]scan-line"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Png {
                handle: _rt::Resource<Png>,
            }
            impl Png {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Png {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[resource-drop]png"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_filter_type(&self) -> FilterType {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.get-filter-type"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        FilterType::_lift(ret as u8)
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_filter_type(&self, t: FilterType) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.set-filter-type"]
                            fn wit_import(_: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, t.clone() as i32);
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn size(&self) -> u32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.size"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u32
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_pixel_at(&self, index: u32) -> u8 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.get-pixel-at"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&index),
                        );
                        ret as u8
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_pixel_at(&self, index: u32, value: u8) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.set-pixel-at"]
                            fn wit_import(_: i32, _: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i32(&index),
                            _rt::as_i32(&value),
                        );
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(&self) -> Result<_rt::Vec<u8>, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.read"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ScanLine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write(&self, pixels: &[u8]) {
                    unsafe {
                        let vec0 = pixels;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]scan-line.write"]
                            fn wit_import(_: i32, _: *mut u8, _: usize);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0);
                    }
                }
            }
            impl Png {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_scan_lines(&self) -> _rt::Vec<ScanLine> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]png.get-scan-lines"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base4 = l1;
                        let len4 = l2;
                        let mut result4 = _rt::Vec::with_capacity(len4);
                        for i in 0..len4 {
                            let base = base4.add(i * 4);
                            let e4 = {
                                let l3 = *base.add(0).cast::<i32>();
                                ScanLine::from_handle(l3 as u32)
                            };
                            result4.push(e4);
                        }
                        _rt::cabi_dealloc(base4, len4 * 4, 4);
                        result4
                    }
                }
            }
            impl Png {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(&self) -> Result<_rt::Vec<u8>, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[method]png.read"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Png {
                #[allow(unused_unsafe, clippy::all)]
                pub fn create(data: &[u8]) -> Result<Png, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let vec0 = data;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(
                            wasm_import_module = "chikoski:glitch-art/png-glitchable@0.3.4"
                        )]
                        extern "C" {
                            #[link_name = "[static]png.create"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<i32>();
                                    Png::from_handle(l3 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod chikoski {
        #[allow(dead_code)]
        pub mod glitch_art {
            #[allow(dead_code, clippy::all)]
            pub mod png_glitchable {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum FilterType {
                    None,
                    Sub,
                    Up,
                    Average,
                    Paeth,
                }
                impl ::core::fmt::Debug for FilterType {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            FilterType::None => {
                                f.debug_tuple("FilterType::None").finish()
                            }
                            FilterType::Sub => f.debug_tuple("FilterType::Sub").finish(),
                            FilterType::Up => f.debug_tuple("FilterType::Up").finish(),
                            FilterType::Average => {
                                f.debug_tuple("FilterType::Average").finish()
                            }
                            FilterType::Paeth => {
                                f.debug_tuple("FilterType::Paeth").finish()
                            }
                        }
                    }
                }
                impl FilterType {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> FilterType {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => FilterType::None,
                            1 => FilterType::Sub,
                            2 => FilterType::Up,
                            3 => FilterType::Average,
                            4 => FilterType::Paeth,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ScanLine {
                    handle: _rt::Resource<ScanLine>,
                }
                type _ScanLineRep<T> = Option<T>;
                impl ScanLine {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `ScanLine`.
                    pub fn new<T: GuestScanLine>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ScanLineRep<T> = Some(val);
                        let ptr: *mut _ScanLineRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestScanLine>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestScanLine>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestScanLine>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ScanLineRep<T>);
                    }
                    fn as_ptr<T: GuestScanLine>(&self) -> *mut _ScanLineRep<T> {
                        ScanLine::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`ScanLine`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ScanLineBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a ScanLine>,
                }
                impl<'a> ScanLineBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestScanLine>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _ScanLineRep<T> {
                        ScanLine::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for ScanLine {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]chikoski:glitch-art/png-glitchable@0.3.4"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]scan-line"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Png {
                    handle: _rt::Resource<Png>,
                }
                type _PngRep<T> = Option<T>;
                impl Png {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Png`.
                    pub fn new<T: GuestPng>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _PngRep<T> = Some(val);
                        let ptr: *mut _PngRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestPng>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestPng>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestPng>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _PngRep<T>);
                    }
                    fn as_ptr<T: GuestPng>(&self) -> *mut _PngRep<T> {
                        Png::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Png`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct PngBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Png>,
                }
                impl<'a> PngBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestPng>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _PngRep<T> {
                        Png::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Png {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]chikoski:glitch-art/png-glitchable@0.3.4"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]png"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_get_filter_type_cabi<
                    T: GuestScanLine,
                >(arg0: *mut u8) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_filter_type(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    result0.clone() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_set_filter_type_cabi<
                    T: GuestScanLine,
                >(arg0: *mut u8, arg1: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_filter_type(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                        FilterType::_lift(arg1 as u8),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_size_cabi<T: GuestScanLine>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::size(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_get_pixel_at_cabi<
                    T: GuestScanLine,
                >(arg0: *mut u8, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_pixel_at(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_set_pixel_at_cabi<
                    T: GuestScanLine,
                >(arg0: *mut u8, arg1: i32, arg2: i32) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::set_pixel_at(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u8,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_read_cabi<T: GuestScanLine>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::read(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        Err(_) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_scan_line_read<T: GuestScanLine>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {}
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_scan_line_write_cabi<T: GuestScanLine>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    T::write(
                        ScanLineBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_png_get_scan_lines_cabi<T: GuestPng>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_scan_lines(
                        PngBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = result0;
                    let len2 = vec2.len();
                    let layout2 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec2.len() * 4,
                        4,
                    );
                    let result2 = if layout2.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout2).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout2);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec2.into_iter().enumerate() {
                        let base = result2.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).take_handle() as i32;
                        }
                    }
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = result2;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_png_get_scan_lines<T: GuestPng>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 4, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_png_read_cabi<T: GuestPng>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::read(PngBorrow::lift(arg0 as u32 as usize).get());
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        Err(_) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_png_read<T: GuestPng>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {}
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_static_png_create_cabi<T: GuestPng>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::create(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(_) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr2
                }
                pub trait Guest {
                    type ScanLine: GuestScanLine;
                    type Png: GuestPng;
                }
                pub trait GuestScanLine: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]chikoski:glitch-art/png-glitchable@0.3.4"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]scan-line"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]chikoski:glitch-art/png-glitchable@0.3.4"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]scan-line"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn get_filter_type(&self) -> FilterType;
                    fn set_filter_type(&self, t: FilterType);
                    fn size(&self) -> u32;
                    fn get_pixel_at(&self, index: u32) -> u8;
                    fn set_pixel_at(&self, index: u32, value: u8);
                    fn read(&self) -> Result<_rt::Vec<u8>, ()>;
                    fn write(&self, pixels: _rt::Vec<u8>);
                }
                pub trait GuestPng: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]chikoski:glitch-art/png-glitchable@0.3.4"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]png"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]chikoski:glitch-art/png-glitchable@0.3.4"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]png"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn get_scan_lines(&self) -> _rt::Vec<ScanLine>;
                    fn read(&self) -> Result<_rt::Vec<u8>, ()>;
                    fn create(data: _rt::Vec<u8>) -> Result<Png, ()>;
                }
                #[doc(hidden)]
                macro_rules! __export_chikoski_glitch_art_png_glitchable_0_3_4_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.get-filter-type"]
                        unsafe extern "C" fn export_method_scan_line_get_filter_type(arg0
                        : * mut u8,) -> i32 { $($path_to_types)*::
                        _export_method_scan_line_get_filter_type_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (arg0) } #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.set-filter-type"]
                        unsafe extern "C" fn export_method_scan_line_set_filter_type(arg0
                        : * mut u8, arg1 : i32,) { $($path_to_types)*::
                        _export_method_scan_line_set_filter_type_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (arg0, arg1) }
                        #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.size"]
                        unsafe extern "C" fn export_method_scan_line_size(arg0 : * mut
                        u8,) -> i32 { $($path_to_types)*::
                        _export_method_scan_line_size_cabi::<<$ty as $($path_to_types)*::
                        Guest >::ScanLine > (arg0) } #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.get-pixel-at"]
                        unsafe extern "C" fn export_method_scan_line_get_pixel_at(arg0 :
                        * mut u8, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_method_scan_line_get_pixel_at_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (arg0, arg1) }
                        #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.set-pixel-at"]
                        unsafe extern "C" fn export_method_scan_line_set_pixel_at(arg0 :
                        * mut u8, arg1 : i32, arg2 : i32,) { $($path_to_types)*::
                        _export_method_scan_line_set_pixel_at_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (arg0, arg1, arg2) }
                        #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.read"]
                        unsafe extern "C" fn export_method_scan_line_read(arg0 : * mut
                        u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_scan_line_read_cabi::<<$ty as $($path_to_types)*::
                        Guest >::ScanLine > (arg0) } #[export_name =
                        "cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.read"]
                        unsafe extern "C" fn _post_return_method_scan_line_read(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_method_scan_line_read::<<$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (arg0) } #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.write"]
                        unsafe extern "C" fn export_method_scan_line_write(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) { $($path_to_types)*::
                        _export_method_scan_line_write_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (arg0, arg1, arg2) }
                        #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]png.get-scan-lines"]
                        unsafe extern "C" fn export_method_png_get_scan_lines(arg0 : *
                        mut u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_png_get_scan_lines_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Png > (arg0) } #[export_name =
                        "cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]png.get-scan-lines"]
                        unsafe extern "C" fn _post_return_method_png_get_scan_lines(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_png_get_scan_lines::<<$ty as
                        $($path_to_types)*:: Guest >::Png > (arg0) } #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[method]png.read"]
                        unsafe extern "C" fn export_method_png_read(arg0 : * mut u8,) ->
                        * mut u8 { $($path_to_types)*::
                        _export_method_png_read_cabi::<<$ty as $($path_to_types)*:: Guest
                        >::Png > (arg0) } #[export_name =
                        "cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]png.read"]
                        unsafe extern "C" fn _post_return_method_png_read(arg0 : * mut
                        u8,) { $($path_to_types)*:: __post_return_method_png_read::<<$ty
                        as $($path_to_types)*:: Guest >::Png > (arg0) } #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[static]png.create"]
                        unsafe extern "C" fn export_static_png_create(arg0 : * mut u8,
                        arg1 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_static_png_create_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Png > (arg0, arg1) } const _ : () = { #[doc(hidden)]
                        #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[dtor]scan-line"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: ScanLine::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::ScanLine > (rep) } }; const _ : ()
                        = { #[doc(hidden)] #[export_name =
                        "chikoski:glitch-art/png-glitchable@0.3.4#[dtor]png"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Png::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Png > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_chikoski_glitch_art_png_glitchable_0_3_4_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod bridge_to_png_glitchable {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Png = super::super::super::super::exports::chikoski::glitch_art::png_glitchable::Png;
                pub type PngBorrow<'a> = super::super::super::super::exports::chikoski::glitch_art::png_glitchable::PngBorrow<
                    'a,
                >;
                pub type ScanLineBorrow<'a> = super::super::super::super::exports::chikoski::glitch_art::png_glitchable::ScanLineBorrow<
                    'a,
                >;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_create_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::create(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                        arg2 as u32,
                        arg3 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(_) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr2
                }
                pub trait Guest {
                    fn create(
                        data: _rt::Vec<u8>,
                        width: u32,
                        height: u32,
                    ) -> Result<Png, ()>;
                }
                #[doc(hidden)]
                macro_rules! __export_chikoski_glitch_art_bridge_to_png_glitchable_0_3_4_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "chikoski:glitch-art/bridge-to-png-glitchable@0.3.4#create"]
                        unsafe extern "C" fn export_create(arg0 : * mut u8, arg1 : usize,
                        arg2 : i32, arg3 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_create_cabi::<$ty > (arg0, arg1, arg2, arg3) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_chikoski_glitch_art_bridge_to_png_glitchable_0_3_4_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_bridge_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::chikoski::glitch_art::png_glitchable::__export_chikoski_glitch_art_png_glitchable_0_3_4_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::chikoski::glitch_art::png_glitchable); $($path_to_types_root)*::
        exports::chikoski::glitch_art::bridge_to_png_glitchable::__export_chikoski_glitch_art_bridge_to_png_glitchable_0_3_4_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::chikoski::glitch_art::bridge_to_png_glitchable);
    };
}
#[doc(inline)]
pub(crate) use __export_bridge_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:chikoski:glitch-art@0.3.4:bridge:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1546] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8d\x0b\x01A\x02\x01\
A\x09\x01B\x20\x01m\x05\x04none\x03sub\x02up\x07average\x05paeth\x04\0\x0bfilter\
-type\x03\0\0\x04\0\x09scan-line\x03\x01\x04\0\x03png\x03\x01\x01h\x02\x01@\x01\x04\
self\x04\0\x01\x04\0![method]scan-line.get-filter-type\x01\x05\x01@\x02\x04self\x04\
\x01t\x01\x01\0\x04\0![method]scan-line.set-filter-type\x01\x06\x01@\x01\x04self\
\x04\0y\x04\0\x16[method]scan-line.size\x01\x07\x01@\x02\x04self\x04\x05indexy\0\
}\x04\0\x1e[method]scan-line.get-pixel-at\x01\x08\x01@\x03\x04self\x04\x05indexy\
\x05value}\x01\0\x04\0\x1e[method]scan-line.set-pixel-at\x01\x09\x01p}\x01j\x01\x0a\
\0\x01@\x01\x04self\x04\0\x0b\x04\0\x16[method]scan-line.read\x01\x0c\x01@\x02\x04\
self\x04\x06pixels\x0a\x01\0\x04\0\x17[method]scan-line.write\x01\x0d\x01h\x03\x01\
i\x02\x01p\x0f\x01@\x01\x04self\x0e\0\x10\x04\0\x1a[method]png.get-scan-lines\x01\
\x11\x01@\x01\x04self\x0e\0\x0b\x04\0\x10[method]png.read\x01\x12\x01i\x03\x01j\x01\
\x13\0\x01@\x01\x04data\x0a\0\x14\x04\0\x12[static]png.create\x01\x15\x03\x01(ch\
ikoski:glitch-art/png-glitchable@0.3.4\x05\0\x01B\x20\x01m\x05\x04none\x03sub\x02\
up\x07average\x05paeth\x04\0\x0bfilter-type\x03\0\0\x04\0\x09scan-line\x03\x01\x04\
\0\x03png\x03\x01\x01h\x02\x01@\x01\x04self\x04\0\x01\x04\0![method]scan-line.ge\
t-filter-type\x01\x05\x01@\x02\x04self\x04\x01t\x01\x01\0\x04\0![method]scan-lin\
e.set-filter-type\x01\x06\x01@\x01\x04self\x04\0y\x04\0\x16[method]scan-line.siz\
e\x01\x07\x01@\x02\x04self\x04\x05indexy\0}\x04\0\x1e[method]scan-line.get-pixel\
-at\x01\x08\x01@\x03\x04self\x04\x05indexy\x05value}\x01\0\x04\0\x1e[method]scan\
-line.set-pixel-at\x01\x09\x01p}\x01j\x01\x0a\0\x01@\x01\x04self\x04\0\x0b\x04\0\
\x16[method]scan-line.read\x01\x0c\x01@\x02\x04self\x04\x06pixels\x0a\x01\0\x04\0\
\x17[method]scan-line.write\x01\x0d\x01h\x03\x01i\x02\x01p\x0f\x01@\x01\x04self\x0e\
\0\x10\x04\0\x1a[method]png.get-scan-lines\x01\x11\x01@\x01\x04self\x0e\0\x0b\x04\
\0\x10[method]png.read\x01\x12\x01i\x03\x01j\x01\x13\0\x01@\x01\x04data\x0a\0\x14\
\x04\0\x12[static]png.create\x01\x15\x04\x01(chikoski:glitch-art/png-glitchable@\
0.3.4\x05\x01\x02\x03\0\x01\x03png\x02\x03\0\x01\x09scan-line\x02\x03\0\x01\x0bf\
ilter-type\x01B\x0b\x02\x03\x02\x01\x02\x04\0\x03png\x03\0\0\x02\x03\x02\x01\x03\
\x04\0\x09scan-line\x03\0\x02\x02\x03\x02\x01\x04\x04\0\x0bfilter-type\x03\0\x04\
\x01p}\x01i\x01\x01j\x01\x07\0\x01@\x03\x04data\x06\x05widthy\x06heighty\0\x08\x04\
\0\x06create\x01\x09\x04\x012chikoski:glitch-art/bridge-to-png-glitchable@0.3.4\x05\
\x05\x04\x01\x20chikoski:glitch-art/bridge@0.3.4\x04\0\x0b\x0c\x01\0\x06bridge\x03\
\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-\
bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
