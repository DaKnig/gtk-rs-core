// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Buildable;
use ColorChooser;
use Container;
use Dialog;
use Widget;
use Window;
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
    pub struct ColorChooserDialog(Object<ffi::GtkColorChooserDialog, ffi::GtkColorChooserDialogClass>): Dialog, Window, Bin, Container, Widget, Buildable, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_chooser_dialog_get_type(),
    }
}

impl ColorChooserDialog {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: IsA<Window> + 'b, R: Into<Option<&'b Q>>>(title: P, parent: R) -> ColorChooserDialog {
        assert_initialized_main_thread!();
        let title = title.into();
        let title = title.to_glib_none();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_chooser_dialog_new(title.0, parent.0)).downcast_unchecked()
        }
    }
}

pub trait ColorChooserDialogExt {
    fn get_property_show_editor(&self) -> bool;

    fn set_property_show_editor(&self, show_editor: bool);

    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorChooserDialog> + IsA<glib::object::Object>> ColorChooserDialogExt for O {
    fn get_property_show_editor(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "show-editor".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "show-editor".to_glib_none().0, Value::from(&show_editor).to_glib_none().0);
        }
    }

    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-editor",
                transmute(notify_show_editor_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_show_editor_trampoline<P>(this: *mut ffi::GtkColorChooserDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooserDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ColorChooserDialog::from_glib_borrow(this).downcast_unchecked())
}
