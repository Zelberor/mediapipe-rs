/* automatically generated by rust-bindgen 0.59.2 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mediagraph_Landmark {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub visibility: f32,
    pub presence: f32,
}
#[test]
fn bindgen_test_layout_mediagraph_Landmark() {
    assert_eq!(
        ::std::mem::size_of::<mediagraph_Landmark>(),
        20usize,
        concat!("Size of: ", stringify!(mediagraph_Landmark))
    );
    assert_eq!(
        ::std::mem::align_of::<mediagraph_Landmark>(),
        4usize,
        concat!("Alignment of ", stringify!(mediagraph_Landmark))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mediagraph_Landmark>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mediagraph_Landmark),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mediagraph_Landmark>())).y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(mediagraph_Landmark),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mediagraph_Landmark>())).z as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mediagraph_Landmark),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mediagraph_Landmark>())).visibility as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(mediagraph_Landmark),
            "::",
            stringify!(visibility)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mediagraph_Landmark>())).presence as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mediagraph_Landmark),
            "::",
            stringify!(presence)
        )
    );
}
pub const mediagraph_DetectorType_POSE: mediagraph_DetectorType = 0;
pub const mediagraph_DetectorType_HANDS: mediagraph_DetectorType = 1;
pub const mediagraph_DetectorType_FACE: mediagraph_DetectorType = 2;
pub type mediagraph_DetectorType = ::std::os::raw::c_uint;
#[repr(C)]
pub struct mediagraph_Detector__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct mediagraph_Detector {
    pub vtable_: *const mediagraph_Detector__bindgen_vtable,
    pub m_graph_type: mediagraph_DetectorType,
}
#[test]
fn bindgen_test_layout_mediagraph_Detector() {
    assert_eq!(
        ::std::mem::size_of::<mediagraph_Detector>(),
        16usize,
        concat!("Size of: ", stringify!(mediagraph_Detector))
    );
    assert_eq!(
        ::std::mem::align_of::<mediagraph_Detector>(),
        8usize,
        concat!("Alignment of ", stringify!(mediagraph_Detector))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mediagraph_Detector>())).m_graph_type as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mediagraph_Detector),
            "::",
            stringify!(m_graph_type)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN10mediagraph8Detector6CreateENS_12DetectorTypeEPKcS3_"]
    pub fn mediagraph_Detector_Create(
        t: mediagraph_DetectorType,
        graph_config: *const ::std::os::raw::c_char,
        output_node: *const ::std::os::raw::c_char,
    ) -> *mut mediagraph_Detector;
}
impl mediagraph_Detector {
    #[inline]
    pub unsafe fn Create(
        t: mediagraph_DetectorType,
        graph_config: *const ::std::os::raw::c_char,
        output_node: *const ::std::os::raw::c_char,
    ) -> *mut mediagraph_Detector {
        mediagraph_Detector_Create(t, graph_config, output_node)
    }
}
extern "C" {
    #[link_name = "\u{1}__ZN10mediagraph8DetectorD1Ev"]
    pub fn mediagraph_Detector_Detector_destructor(this: *mut mediagraph_Detector);
}
extern "C" {
    #[link_name = "\u{1}__ZN10mediagraph8Detector7ProcessEPhii"]
    pub fn mediagraph_Detector_Process(
        this: *mut ::std::os::raw::c_void,
        data: *mut u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> *mut mediagraph_Landmark;
}
#[repr(C)]
pub struct mediagraph_Effect__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct mediagraph_Effect {
    pub vtable_: *const mediagraph_Effect__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_mediagraph_Effect() {
    assert_eq!(
        ::std::mem::size_of::<mediagraph_Effect>(),
        8usize,
        concat!("Size of: ", stringify!(mediagraph_Effect))
    );
    assert_eq!(
        ::std::mem::align_of::<mediagraph_Effect>(),
        8usize,
        concat!("Alignment of ", stringify!(mediagraph_Effect))
    );
}
extern "C" {
    #[link_name = "\u{1}__ZN10mediagraph6Effect6CreateEPKcS2_"]
    pub fn mediagraph_Effect_Create(
        graph_config: *const ::std::os::raw::c_char,
        output_node: *const ::std::os::raw::c_char,
    ) -> *mut mediagraph_Effect;
}
impl mediagraph_Effect {
    #[inline]
    pub unsafe fn Create(
        graph_config: *const ::std::os::raw::c_char,
        output_node: *const ::std::os::raw::c_char,
    ) -> *mut mediagraph_Effect {
        mediagraph_Effect_Create(graph_config, output_node)
    }
}
extern "C" {
    #[link_name = "\u{1}__ZN10mediagraph6EffectD1Ev"]
    pub fn mediagraph_Effect_Effect_destructor(this: *mut mediagraph_Effect);
}
extern "C" {
    #[link_name = "\u{1}__ZN10mediagraph6Effect7ProcessEPhii"]
    pub fn mediagraph_Effect_Process(
        this: *mut ::std::os::raw::c_void,
        data: *mut u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> *mut u8;
}
