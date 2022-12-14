// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NemoPropertyPage")]
    pub struct PropertyPage(Object<ffi::NemoPropertyPage, ffi::NemoPropertyPageClass>);

    match fn {
        type_ => || ffi::nemo_property_page_get_type(),
    }
}

impl PropertyPage {
    #[doc(alias = "nemo_property_page_new")]
    pub fn new(name: &str, label: &impl IsA<gtk::Widget>, page: &impl IsA<gtk::Widget>) -> PropertyPage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::nemo_property_page_new(name.to_glib_none().0, label.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`PropertyPage`] objects.
            ///
            /// This method returns an instance of [`PropertyPageBuilder`](crate::builders::PropertyPageBuilder) which can be used to create [`PropertyPage`] objects.
            pub fn builder() -> PropertyPageBuilder {
                PropertyPageBuilder::default()
            }
        

    pub fn label(&self) -> Option<gtk::Widget> {
        glib::ObjectExt::property(self, "label")
    }

    pub fn set_label<P: IsA<gtk::Widget>>(&self, label: Option<&P>) {
        glib::ObjectExt::set_property(self,"label", &label)
    }

    pub fn name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "name")
    }

    pub fn page(&self) -> Option<gtk::Widget> {
        glib::ObjectExt::property(self, "page")
    }

    pub fn set_page<P: IsA<gtk::Widget>>(&self, page: Option<&P>) {
        glib::ObjectExt::set_property(self,"page", &page)
    }

    #[doc(alias = "label")]
    pub fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&PropertyPage) + 'static>(this: *mut ffi::NemoPropertyPage, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_label_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "page")]
    pub fn connect_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<F: Fn(&PropertyPage) + 'static>(this: *mut ffi::NemoPropertyPage, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_page_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for PropertyPage {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>(&[])
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`PropertyPage`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PropertyPageBuilder {
    label: Option<gtk::Widget>,
    name: Option<String>,
    page: Option<gtk::Widget>,
}

impl PropertyPageBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PropertyPageBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`PropertyPage`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PropertyPage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref label) = self.label {
                properties.push(("label", label));
            }
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
if let Some(ref page) = self.page {
                properties.push(("page", page));
            }
        glib::Object::new::<PropertyPage>(&properties)

    }

    pub fn label(mut self, label: &impl IsA<gtk::Widget>) -> Self {
        self.label = Some(label.clone().upcast());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn page(mut self, page: &impl IsA<gtk::Widget>) -> Self {
        self.page = Some(page.clone().upcast());
        self
    }
}

impl fmt::Display for PropertyPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PropertyPage")
    }
}
