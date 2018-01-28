// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Border;
use CssSection;
use IconSet;
use JunctionSides;
use RegionFlags;
use StateFlags;
use StateType;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use StyleContextPrintFlags;
use StyleProvider;
use TextDirection;
use ffi;
use gdk;
use glib;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_8", feature = "dox"))]
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
    pub struct StyleContext(Object<ffi::GtkStyleContext, ffi::GtkStyleContextClass>);

    match fn {
        get_type => || ffi::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    pub fn new() -> StyleContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_style_context_new())
        }
    }

    pub fn add_provider_for_screen<P: IsA<StyleProvider>>(screen: &gdk::Screen, provider: &P, priority: u32) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_add_provider_for_screen(screen.to_glib_none().0, provider.to_glib_none().0, priority);
        }
    }

    pub fn remove_provider_for_screen<P: IsA<StyleProvider>>(screen: &gdk::Screen, provider: &P) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_remove_provider_for_screen(screen.to_glib_none().0, provider.to_glib_none().0);
        }
    }

    pub fn reset_widgets(screen: &gdk::Screen) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_style_context_reset_widgets(screen.to_glib_none().0);
        }
    }
}

impl Default for StyleContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StyleContextExt {
    fn add_class(&self, class_name: &str);

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn add_region(&self, region_name: &str, flags: RegionFlags);

    //#[cfg_attr(feature = "v3_6", deprecated)]
    //fn cancel_animations<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, region_id: P);

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_background_color(&self, state: StateFlags) -> gdk::RGBA;

    fn get_border(&self, state: StateFlags) -> Border;

    #[cfg_attr(feature = "v3_16", deprecated)]
    fn get_border_color(&self, state: StateFlags) -> gdk::RGBA;

    fn get_color(&self, state: StateFlags) -> gdk::RGBA;

    #[cfg_attr(feature = "v3_8", deprecated)]
    fn get_direction(&self) -> TextDirection;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_frame_clock(&self) -> Option<gdk::FrameClock>;

    fn get_junction_sides(&self) -> JunctionSides;

    fn get_margin(&self, state: StateFlags) -> Border;

    fn get_padding(&self, state: StateFlags) -> Border;

    fn get_parent(&self) -> Option<StyleContext>;

    //fn get_path(&self) -> /*Ignored*/Option<WidgetPath>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_scale(&self) -> i32;

    fn get_screen(&self) -> Option<gdk::Screen>;

    fn get_section(&self, property: &str) -> Option<CssSection>;

    fn get_state(&self) -> StateFlags;

    //fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    fn has_class(&self, class_name: &str) -> bool;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn has_region(&self, region_name: &str) -> Option<RegionFlags>;

    #[cfg_attr(feature = "v3_12", deprecated)]
    fn invalidate(&self);

    fn list_classes(&self) -> Vec<String>;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn list_regions(&self) -> Vec<String>;

    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA>;

    #[cfg_attr(feature = "v3_10", deprecated)]
    fn lookup_icon_set(&self, stock_id: &str) -> Option<IconSet>;

    //#[cfg_attr(feature = "v3_6", deprecated)]
    //fn notify_state_change<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, window: &gdk::Window, region_id: P, state: StateType, state_value: bool);

    #[cfg_attr(feature = "v3_6", deprecated)]
    fn pop_animatable_region(&self);

    //#[cfg_attr(feature = "v3_6", deprecated)]
    //fn push_animatable_region<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, region_id: P);

    fn remove_class(&self, class_name: &str);

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P);

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn remove_region(&self, region_name: &str);

    fn restore(&self);

    fn save(&self);

    #[cfg_attr(feature = "v3_6", deprecated)]
    fn scroll_animations(&self, window: &gdk::Window, dx: i32, dy: i32);

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn set_background(&self, window: &gdk::Window);

    #[cfg_attr(feature = "v3_8", deprecated)]
    fn set_direction(&self, direction: TextDirection);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_frame_clock(&self, frame_clock: &gdk::FrameClock);

    fn set_junction_sides(&self, sides: JunctionSides);

    fn set_parent<'a, P: Into<Option<&'a StyleContext>>>(&self, parent: P);

    //fn set_path(&self, path: /*Ignored*/&mut WidgetPath);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_scale(&self, scale: i32);

    fn set_screen(&self, screen: &gdk::Screen);

    fn set_state(&self, flags: StateFlags);

    #[cfg_attr(feature = "v3_6", deprecated)]
    fn state_is_running(&self, state: StateType) -> Option<f64>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn to_string(&self, flags: StyleContextPrintFlags) -> String;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_property_paint_clock(&self) -> Option<gdk::FrameClock>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn connect_property_paint_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleContext> + IsA<glib::object::Object>> StyleContextExt for O {
    fn add_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_add_class(self.to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32) {
        unsafe {
            ffi::gtk_style_context_add_provider(self.to_glib_none().0, provider.to_glib_none().0, priority);
        }
    }

    fn add_region(&self, region_name: &str, flags: RegionFlags) {
        unsafe {
            ffi::gtk_style_context_add_region(self.to_glib_none().0, region_name.to_glib_none().0, flags.to_glib());
        }
    }

    //fn cancel_animations<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, region_id: P) {
    //    unsafe { TODO: call ffi::gtk_style_context_cancel_animations() }
    //}

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_context_get() }
    //}

    fn get_background_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_background_color(self.to_glib_none().0, state.to_glib(), color.to_glib_none_mut().0);
            color
        }
    }

    fn get_border(&self, state: StateFlags) -> Border {
        unsafe {
            let mut border = Border::uninitialized();
            ffi::gtk_style_context_get_border(self.to_glib_none().0, state.to_glib(), border.to_glib_none_mut().0);
            border
        }
    }

    fn get_border_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_border_color(self.to_glib_none().0, state.to_glib(), color.to_glib_none_mut().0);
            color
        }
    }

    fn get_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_color(self.to_glib_none().0, state.to_glib(), color.to_glib_none_mut().0);
            color
        }
    }

    fn get_direction(&self) -> TextDirection {
        unsafe {
            from_glib(ffi::gtk_style_context_get_direction(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_frame_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_frame_clock(self.to_glib_none().0))
        }
    }

    fn get_junction_sides(&self) -> JunctionSides {
        unsafe {
            from_glib(ffi::gtk_style_context_get_junction_sides(self.to_glib_none().0))
        }
    }

    fn get_margin(&self, state: StateFlags) -> Border {
        unsafe {
            let mut margin = Border::uninitialized();
            ffi::gtk_style_context_get_margin(self.to_glib_none().0, state.to_glib(), margin.to_glib_none_mut().0);
            margin
        }
    }

    fn get_padding(&self, state: StateFlags) -> Border {
        unsafe {
            let mut padding = Border::uninitialized();
            ffi::gtk_style_context_get_padding(self.to_glib_none().0, state.to_glib(), padding.to_glib_none_mut().0);
            padding
        }
    }

    fn get_parent(&self) -> Option<StyleContext> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_parent(self.to_glib_none().0))
        }
    }

    //fn get_path(&self) -> /*Ignored*/Option<WidgetPath> {
    //    unsafe { TODO: call ffi::gtk_style_context_get_path() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_scale(&self) -> i32 {
        unsafe {
            ffi::gtk_style_context_get_scale(self.to_glib_none().0)
        }
    }

    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_screen(self.to_glib_none().0))
        }
    }

    fn get_section(&self, property: &str) -> Option<CssSection> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_section(self.to_glib_none().0, property.to_glib_none().0))
        }
    }

    fn get_state(&self) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_style_context_get_state(self.to_glib_none().0))
        }
    }

    //fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style() }
    //}

    //fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style_valist() }
    //}

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_valist() }
    //}

    fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_style_context_has_class(self.to_glib_none().0, class_name.to_glib_none().0))
        }
    }

    fn has_region(&self, region_name: &str) -> Option<RegionFlags> {
        unsafe {
            let mut flags_return = mem::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_has_region(self.to_glib_none().0, region_name.to_glib_none().0, &mut flags_return));
            if ret { Some(from_glib(flags_return)) } else { None }
        }
    }

    fn invalidate(&self) {
        unsafe {
            ffi::gtk_style_context_invalidate(self.to_glib_none().0);
        }
    }

    fn list_classes(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_style_context_list_classes(self.to_glib_none().0))
        }
    }

    fn list_regions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_style_context_list_regions(self.to_glib_none().0))
        }
    }

    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_lookup_color(self.to_glib_none().0, color_name.to_glib_none().0, color.to_glib_none_mut().0));
            if ret { Some(color) } else { None }
        }
    }

    fn lookup_icon_set(&self, stock_id: &str) -> Option<IconSet> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_lookup_icon_set(self.to_glib_none().0, stock_id.to_glib_none().0))
        }
    }

    //fn notify_state_change<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, window: &gdk::Window, region_id: P, state: StateType, state_value: bool) {
    //    unsafe { TODO: call ffi::gtk_style_context_notify_state_change() }
    //}

    fn pop_animatable_region(&self) {
        unsafe {
            ffi::gtk_style_context_pop_animatable_region(self.to_glib_none().0);
        }
    }

    //fn push_animatable_region<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, region_id: P) {
    //    unsafe { TODO: call ffi::gtk_style_context_push_animatable_region() }
    //}

    fn remove_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_class(self.to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P) {
        unsafe {
            ffi::gtk_style_context_remove_provider(self.to_glib_none().0, provider.to_glib_none().0);
        }
    }

    fn remove_region(&self, region_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_region(self.to_glib_none().0, region_name.to_glib_none().0);
        }
    }

    fn restore(&self) {
        unsafe {
            ffi::gtk_style_context_restore(self.to_glib_none().0);
        }
    }

    fn save(&self) {
        unsafe {
            ffi::gtk_style_context_save(self.to_glib_none().0);
        }
    }

    fn scroll_animations(&self, window: &gdk::Window, dx: i32, dy: i32) {
        unsafe {
            ffi::gtk_style_context_scroll_animations(self.to_glib_none().0, window.to_glib_none().0, dx, dy);
        }
    }

    fn set_background(&self, window: &gdk::Window) {
        unsafe {
            ffi::gtk_style_context_set_background(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    fn set_direction(&self, direction: TextDirection) {
        unsafe {
            ffi::gtk_style_context_set_direction(self.to_glib_none().0, direction.to_glib());
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_frame_clock(&self, frame_clock: &gdk::FrameClock) {
        unsafe {
            ffi::gtk_style_context_set_frame_clock(self.to_glib_none().0, frame_clock.to_glib_none().0);
        }
    }

    fn set_junction_sides(&self, sides: JunctionSides) {
        unsafe {
            ffi::gtk_style_context_set_junction_sides(self.to_glib_none().0, sides.to_glib());
        }
    }

    fn set_parent<'a, P: Into<Option<&'a StyleContext>>>(&self, parent: P) {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            ffi::gtk_style_context_set_parent(self.to_glib_none().0, parent.0);
        }
    }

    //fn set_path(&self, path: /*Ignored*/&mut WidgetPath) {
    //    unsafe { TODO: call ffi::gtk_style_context_set_path() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_scale(&self, scale: i32) {
        unsafe {
            ffi::gtk_style_context_set_scale(self.to_glib_none().0, scale);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_style_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn set_state(&self, flags: StateFlags) {
        unsafe {
            ffi::gtk_style_context_set_state(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn state_is_running(&self, state: StateType) -> Option<f64> {
        unsafe {
            let mut progress = mem::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_state_is_running(self.to_glib_none().0, state.to_glib(), &mut progress));
            if ret { Some(progress) } else { None }
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn to_string(&self, flags: StyleContextPrintFlags) -> String {
        unsafe {
            from_glib_full(ffi::gtk_style_context_to_string(self.to_glib_none().0, flags.to_glib()))
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_property_paint_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            let mut value = Value::from_type(<gdk::FrameClock as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "paint-clock".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "paint-clock".to_glib_none().0, Value::from(paint_clock).to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::direction",
                transmute(notify_direction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn connect_property_paint_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::paint-clock",
                transmute(notify_paint_clock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parent",
                transmute(notify_parent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::screen",
                transmute(notify_screen_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkStyleContext, f: glib_ffi::gpointer)
where P: IsA<StyleContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_direction_trampoline<P>(this: *mut ffi::GtkStyleContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_8", feature = "dox"))]
unsafe extern "C" fn notify_paint_clock_trampoline<P>(this: *mut ffi::GtkStyleContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_parent_trampoline<P>(this: *mut ffi::GtkStyleContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_screen_trampoline<P>(this: *mut ffi::GtkStyleContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StyleContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StyleContext::from_glib_borrow(this).downcast_unchecked())
}
