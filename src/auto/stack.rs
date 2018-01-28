// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Buildable;
use Container;
use StackTransitionType;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Stack(Object<ffi::GtkStack, ffi::GtkStackClass>): Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StackExt {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn add_named<P: IsA<Widget>>(&self, child: &P, name: &str);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn add_titled<P: IsA<Widget>>(&self, child: &P, name: &str, title: &str);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_child_by_name(&self, name: &str) -> Option<Widget>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_hhomogeneous(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_homogeneous(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_interpolate_size(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_duration(&self) -> u32;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_transition_running(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_type(&self) -> StackTransitionType;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_vhomogeneous(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_visible_child(&self) -> Option<Widget>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_visible_child_name(&self) -> Option<String>;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_hhomogeneous(&self, hhomogeneous: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_homogeneous(&self, homogeneous: bool);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_interpolate_size(&self, interpolate_size: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_duration(&self, duration: u32);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_type(&self, transition: StackTransitionType);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_vhomogeneous(&self, vhomogeneous: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_visible_child<P: IsA<Widget>>(&self, child: &P);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_visible_child_name(&self, name: &str);

    fn get_property_homogeneous(&self) -> bool;

    fn set_property_homogeneous(&self, homogeneous: bool);

    fn get_property_interpolate_size(&self) -> bool;

    fn set_property_interpolate_size(&self, interpolate_size: bool);

    fn get_property_transition_duration(&self) -> u32;

    fn set_property_transition_duration(&self, transition_duration: u32);

    fn get_property_transition_running(&self) -> bool;

    fn get_property_transition_type(&self) -> StackTransitionType;

    fn set_property_transition_type(&self, transition_type: StackTransitionType);

    fn get_property_visible_child(&self) -> Option<Widget>;

    fn set_property_visible_child<P: IsA<Widget> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, visible_child: Option<&P>);

    fn get_property_visible_child_name(&self) -> Option<String>;

    fn set_property_visible_child_name(&self, visible_child_name: Option<&str>);

    fn get_child_icon_name<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_icon_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, icon_name: P);

    fn get_child_name<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, name: P);

    fn get_child_needs_attention<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_needs_attention<T: IsA<Widget>>(&self, item: &T, needs_attention: bool);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_child_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Stack> + IsA<Container> + IsA<glib::object::Object>> StackExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn add_named<P: IsA<Widget>>(&self, child: &P, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn add_titled<P: IsA<Widget>>(&self, child: &P, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_hhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_hhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_homogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_interpolate_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_interpolate_size(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_running(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_transition_type(&self) -> StackTransitionType {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_vhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_vhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_visible_child_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::gtk_stack_set_interpolate_size(self.to_glib_none().0, interpolate_size.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_visible_child<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_stack_set_visible_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(self.to_glib_none().0, name.to_glib_none().0, transition.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn get_property_homogeneous(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "homogeneous".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "homogeneous".to_glib_none().0, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_property_interpolate_size(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "interpolate-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "interpolate-size".to_glib_none().0, Value::from(&interpolate_size).to_glib_none().0);
        }
    }

    fn get_property_transition_duration(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_transition_duration(&self, transition_duration: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, Value::from(&transition_duration).to_glib_none().0);
        }
    }

    fn get_property_transition_running(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-running".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_transition_type(&self) -> StackTransitionType {
        unsafe {
            let mut value = Value::from_type(<StackTransitionType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_transition_type(&self, transition_type: StackTransitionType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-type".to_glib_none().0, Value::from(&transition_type).to_glib_none().0);
        }
    }

    fn get_property_visible_child(&self) -> Option<Widget> {
        unsafe {
            let mut value = Value::from_type(<Widget as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-child".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_visible_child<P: IsA<Widget> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, visible_child: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-child".to_glib_none().0, Value::from(visible_child).to_glib_none().0);
        }
    }

    fn get_property_visible_child_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-child-name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_visible_child_name(&self, visible_child_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-child-name".to_glib_none().0, Value::from(visible_child_name).to_glib_none().0);
        }
    }

    fn get_child_icon_name<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "icon-name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_child_icon_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, icon_name: P) {
        let icon_name = icon_name.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_child_name<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_child_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, name: P) {
        let name = name.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "name".to_glib_none().0, Value::from(name).to_glib_none().0);
        }
    }

    fn get_child_needs_attention<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "needs-attention".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_needs_attention<T: IsA<Widget>>(&self, item: &T, needs_attention: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "needs-attention".to_glib_none().0, Value::from(&needs_attention).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P) {
        let title = title.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, Value::from(title).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hhomogeneous",
                transmute(notify_hhomogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::homogeneous",
                transmute(notify_homogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::interpolate-size",
                transmute(notify_interpolate_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transition-duration",
                transmute(notify_transition_duration_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transition-running",
                transmute(notify_transition_running_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transition-type",
                transmute(notify_transition_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::vhomogeneous",
                transmute(notify_vhomogeneous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible-child",
                transmute(notify_visible_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_child_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible-child-name",
                transmute(notify_visible_child_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_hhomogeneous_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_homogeneous_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_interpolate_size_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transition_duration_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transition_running_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transition_type_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_vhomogeneous_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_child_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_child_name_trampoline<P>(this: *mut ffi::GtkStack, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Stack> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Stack::from_glib_borrow(this).downcast_unchecked())
}
