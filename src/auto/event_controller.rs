// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use PropagationPhase;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gdk;
use glib;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct EventController(Object<ffi::GtkEventController, ffi::GtkEventControllerClass>);

    match fn {
        get_type => || ffi::gtk_event_controller_get_type(),
    }
}

pub trait EventControllerExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_propagation_phase(&self) -> PropagationPhase;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_widget(&self) -> Option<Widget>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn reset(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_propagation_phase(&self, phase: PropagationPhase);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_propagation_phase_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EventController> + IsA<glib::object::Object>> EventControllerExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_propagation_phase(&self) -> PropagationPhase {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_propagation_phase(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_widget(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_handle_event(self.to_glib_none().0, event.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn reset(&self) {
        unsafe {
            ffi::gtk_event_controller_reset(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_propagation_phase(&self, phase: PropagationPhase) {
        unsafe {
            ffi::gtk_event_controller_set_propagation_phase(self.to_glib_none().0, phase.to_glib());
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_propagation_phase_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::propagation-phase",
                transmute(notify_propagation_phase_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::widget",
                transmute(notify_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_propagation_phase_trampoline<P>(this: *mut ffi::GtkEventController, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EventController> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EventController::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_widget_trampoline<P>(this: *mut ffi::GtkEventController, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EventController> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&EventController::from_glib_borrow(this).downcast_unchecked())
}
