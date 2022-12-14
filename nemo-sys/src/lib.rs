// Generated by gir (https://github.com/gtk-rs/gir @ b5068ede6c51)
// from ../../gir-files (@ 248ad6976459)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]


#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Aliases
pub type NemoColumnProviderIface = NemoColumnProviderInterface;
pub type NemoInfoProviderIface = NemoInfoProviderInterface;
pub type NemoLocationWidgetProviderIface = NemoLocationWidgetProviderInterface;
pub type NemoMenuProviderIface = NemoMenuProviderInterface;
pub type NemoNameAndDescProviderIface = NemoNameAndDescProviderInterface;
pub type NemoPropertyPageProviderIface = NemoPropertyPageProviderInterface;

// Enums
pub type NemoOperationResult = c_int;
pub const NEMO_OPERATION_COMPLETE: NemoOperationResult = 0;
pub const NEMO_OPERATION_FAILED: NemoOperationResult = 1;
pub const NEMO_OPERATION_IN_PROGRESS: NemoOperationResult = 2;

// Callbacks
pub type NemoInfoProviderUpdateComplete = Option<unsafe extern "C" fn(*mut NemoInfoProvider, *mut NemoOperationHandle, NemoOperationResult, gpointer)>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoColumnClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for NemoColumnClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoColumnClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoColumnProviderInterface {
    pub g_iface: gobject::GTypeInterface,
    pub get_columns: Option<unsafe extern "C" fn(*mut NemoColumnProvider) -> *mut glib::GList>,
}

impl ::std::fmt::Debug for NemoColumnProviderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoColumnProviderInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("get_columns", &self.get_columns)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoDesktopPreferencesClass {
    pub parent_class: gtk::GtkBinClass,
}

impl ::std::fmt::Debug for NemoDesktopPreferencesClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoDesktopPreferencesClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _NemoFile {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type NemoFile = *mut _NemoFile;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoFileInfoInterface {
    pub g_iface: gobject::GTypeInterface,
    pub is_gone: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> gboolean>,
    pub get_name: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut c_char>,
    pub get_uri: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut c_char>,
    pub get_parent_uri: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut c_char>,
    pub get_uri_scheme: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut c_char>,
    pub get_mime_type: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut c_char>,
    pub is_mime_type: Option<unsafe extern "C" fn(*mut NemoFileInfo, *const c_char) -> gboolean>,
    pub is_directory: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> gboolean>,
    pub add_emblem: Option<unsafe extern "C" fn(*mut NemoFileInfo, *const c_char)>,
    pub get_string_attribute: Option<unsafe extern "C" fn(*mut NemoFileInfo, *const c_char) -> *mut c_char>,
    pub add_string_attribute: Option<unsafe extern "C" fn(*mut NemoFileInfo, *const c_char, *const c_char)>,
    pub invalidate_extension_info: Option<unsafe extern "C" fn(*mut NemoFileInfo)>,
    pub get_activation_uri: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut c_char>,
    pub get_file_type: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> gio::GFileType>,
    pub get_location: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut gio::GFile>,
    pub get_parent_location: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut gio::GFile>,
    pub get_parent_info: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut NemoFileInfo>,
    pub get_mount: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> *mut gio::GMount>,
    pub can_write: Option<unsafe extern "C" fn(*mut NemoFileInfo) -> gboolean>,
}

impl ::std::fmt::Debug for NemoFileInfoInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoFileInfoInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("is_gone", &self.is_gone)
         .field("get_name", &self.get_name)
         .field("get_uri", &self.get_uri)
         .field("get_parent_uri", &self.get_parent_uri)
         .field("get_uri_scheme", &self.get_uri_scheme)
         .field("get_mime_type", &self.get_mime_type)
         .field("is_mime_type", &self.is_mime_type)
         .field("is_directory", &self.is_directory)
         .field("add_emblem", &self.add_emblem)
         .field("get_string_attribute", &self.get_string_attribute)
         .field("add_string_attribute", &self.add_string_attribute)
         .field("invalidate_extension_info", &self.invalidate_extension_info)
         .field("get_activation_uri", &self.get_activation_uri)
         .field("get_file_type", &self.get_file_type)
         .field("get_location", &self.get_location)
         .field("get_parent_location", &self.get_parent_location)
         .field("get_parent_info", &self.get_parent_info)
         .field("get_mount", &self.get_mount)
         .field("can_write", &self.can_write)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoInfoProviderInterface {
    pub g_iface: gobject::GTypeInterface,
    pub update_file_info: Option<unsafe extern "C" fn(*mut NemoInfoProvider, *mut NemoFileInfo, *mut gobject::GClosure, *mut *mut NemoOperationHandle) -> NemoOperationResult>,
    pub cancel_update: Option<unsafe extern "C" fn(*mut NemoInfoProvider, *mut NemoOperationHandle)>,
}

impl ::std::fmt::Debug for NemoInfoProviderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoInfoProviderInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("update_file_info", &self.update_file_info)
         .field("cancel_update", &self.cancel_update)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoLocationWidgetProviderInterface {
    pub g_iface: gobject::GTypeInterface,
    pub get_widget: Option<unsafe extern "C" fn(*mut NemoLocationWidgetProvider, *const c_char, *mut gtk::GtkWidget) -> *mut gtk::GtkWidget>,
}

impl ::std::fmt::Debug for NemoLocationWidgetProviderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoLocationWidgetProviderInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("get_widget", &self.get_widget)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoMenuClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for NemoMenuClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoMenuClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoMenuItemClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for NemoMenuItemClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoMenuItemClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoMenuProviderInterface {
    pub g_iface: gobject::GTypeInterface,
    pub get_file_items: Option<unsafe extern "C" fn(*mut NemoMenuProvider, *mut gtk::GtkWidget, *mut glib::GList) -> *mut glib::GList>,
    pub get_background_items: Option<unsafe extern "C" fn(*mut NemoMenuProvider, *mut gtk::GtkWidget, *mut NemoFileInfo) -> *mut glib::GList>,
}

impl ::std::fmt::Debug for NemoMenuProviderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoMenuProviderInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("get_file_items", &self.get_file_items)
         .field("get_background_items", &self.get_background_items)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoNameAndDescProviderInterface {
    pub g_iface: gobject::GTypeInterface,
    pub get_name_and_desc: Option<unsafe extern "C" fn(*mut NemoNameAndDescProvider) -> *mut glib::GList>,
}

impl ::std::fmt::Debug for NemoNameAndDescProviderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoNameAndDescProviderInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("get_name_and_desc", &self.get_name_and_desc)
         .finish()
    }
}

#[repr(C)]
pub struct _NemoOperationHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type NemoOperationHandle = *mut _NemoOperationHandle;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoPropertyPageClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for NemoPropertyPageClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoPropertyPageClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoPropertyPageProviderInterface {
    pub g_iface: gobject::GTypeInterface,
    pub get_pages: Option<unsafe extern "C" fn(*mut NemoPropertyPageProvider, *mut glib::GList) -> *mut glib::GList>,
}

impl ::std::fmt::Debug for NemoPropertyPageProviderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoPropertyPageProviderInterface @ {:p}", self))
         .field("g_iface", &self.g_iface)
         .field("get_pages", &self.get_pages)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NemoSimpleButtonClass {
    pub parent_class: gtk::GtkButtonClass,
}

impl ::std::fmt::Debug for NemoSimpleButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoSimpleButtonClass @ {:p}", self))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[repr(C)]
pub struct NemoColumn {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoColumn @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct NemoDesktopPreferences {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoDesktopPreferences {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoDesktopPreferences @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct NemoMenu {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoMenu @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct NemoMenuItem {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoMenuItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoMenuItem @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct NemoPropertyPage {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoPropertyPage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoPropertyPage @ {:p}", self))
         .finish()
    }
}

#[repr(C)]
pub struct NemoSimpleButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoSimpleButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("NemoSimpleButton @ {:p}", self))
         .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct NemoColumnProvider {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoColumnProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoColumnProvider @ {:p}", self)
    }
}

#[repr(C)]
pub struct NemoFileInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoFileInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoFileInfo @ {:p}", self)
    }
}

#[repr(C)]
pub struct NemoInfoProvider {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoInfoProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoInfoProvider @ {:p}", self)
    }
}

#[repr(C)]
pub struct NemoLocationWidgetProvider {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoLocationWidgetProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoLocationWidgetProvider @ {:p}", self)
    }
}

#[repr(C)]
pub struct NemoMenuProvider {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoMenuProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoMenuProvider @ {:p}", self)
    }
}

#[repr(C)]
pub struct NemoNameAndDescProvider {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoNameAndDescProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoNameAndDescProvider @ {:p}", self)
    }
}

#[repr(C)]
pub struct NemoPropertyPageProvider {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for NemoPropertyPageProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NemoPropertyPageProvider @ {:p}", self)
    }
}


#[link(name = "nemo-extension")]
extern "C" {

    //=========================================================================
    // NemoColumn
    //=========================================================================
    pub fn nemo_column_get_type() -> GType;
    pub fn nemo_column_new(name: *const c_char, attribute: *const c_char, label: *const c_char, description: *const c_char) -> *mut NemoColumn;
    pub fn nemo_column_new2(name: *const c_char, attribute: *const c_char, label: *const c_char, description: *const c_char, width_chars: c_int, ellipsize: pango::PangoEllipsizeMode) -> *mut NemoColumn;

    //=========================================================================
    // NemoDesktopPreferences
    //=========================================================================
    pub fn nemo_desktop_preferences_get_type() -> GType;
    pub fn nemo_desktop_preferences_new() -> *mut NemoDesktopPreferences;

    //=========================================================================
    // NemoMenu
    //=========================================================================
    pub fn nemo_menu_get_type() -> GType;
    pub fn nemo_menu_new() -> *mut NemoMenu;
    pub fn nemo_menu_append_item(menu: *mut NemoMenu, item: *mut NemoMenuItem);
    pub fn nemo_menu_get_items(menu: *mut NemoMenu) -> *mut glib::GList;

    //=========================================================================
    // NemoMenuItem
    //=========================================================================
    pub fn nemo_menu_item_get_type() -> GType;
    pub fn nemo_menu_item_new(name: *const c_char, label: *const c_char, tip: *const c_char, icon: *const c_char) -> *mut NemoMenuItem;
    pub fn nemo_menu_item_new_separator(name: *const c_char) -> *mut NemoMenuItem;
    pub fn nemo_menu_item_new_widget(name: *const c_char, widget_a: *mut gtk::GtkWidget, widget_b: *mut gtk::GtkWidget) -> *mut NemoMenuItem;
    pub fn nemo_menu_item_list_free(item_list: *mut glib::GList);
    pub fn nemo_menu_item_activate(item: *mut NemoMenuItem);
    pub fn nemo_menu_item_set_submenu(item: *mut NemoMenuItem, menu: *mut NemoMenu);
    pub fn nemo_menu_item_set_widget_a(item: *mut NemoMenuItem, widget: *mut gtk::GtkWidget);
    pub fn nemo_menu_item_set_widget_b(item: *mut NemoMenuItem, widget: *mut gtk::GtkWidget);

    //=========================================================================
    // NemoPropertyPage
    //=========================================================================
    pub fn nemo_property_page_get_type() -> GType;
    pub fn nemo_property_page_new(name: *const c_char, label: *mut gtk::GtkWidget, page: *mut gtk::GtkWidget) -> *mut NemoPropertyPage;

    //=========================================================================
    // NemoSimpleButton
    //=========================================================================
    pub fn nemo_simple_button_get_type() -> GType;
    pub fn nemo_simple_button_new() -> *mut NemoSimpleButton;
    pub fn nemo_simple_button_new_from_file(path: *const c_char, icon_size: c_int) -> *mut NemoSimpleButton;
    pub fn nemo_simple_button_new_from_icon_name(icon_name: *const c_char, icon_size: c_int) -> *mut NemoSimpleButton;
    pub fn nemo_simple_button_new_from_stock(stock_id: *const c_char, icon_size: c_int) -> *mut NemoSimpleButton;

    //=========================================================================
    // NemoColumnProvider
    //=========================================================================
    pub fn nemo_column_provider_get_type() -> GType;
    pub fn nemo_column_provider_get_columns(provider: *mut NemoColumnProvider) -> *mut glib::GList;

    //=========================================================================
    // NemoFileInfo
    //=========================================================================
    pub fn nemo_file_info_get_type() -> GType;
    pub fn nemo_file_info_create(location: *mut gio::GFile) -> *mut NemoFileInfo;
    pub fn nemo_file_info_create_for_uri(uri: *const c_char) -> *mut NemoFileInfo;
    pub fn nemo_file_info_list_copy(files: *mut glib::GList) -> *mut glib::GList;
    pub fn nemo_file_info_list_free(files: *mut glib::GList);
    pub fn nemo_file_info_lookup(location: *mut gio::GFile) -> *mut NemoFileInfo;
    pub fn nemo_file_info_lookup_for_uri(uri: *const c_char) -> *mut NemoFileInfo;
    pub fn nemo_file_info_add_emblem(file: *mut NemoFileInfo, emblem_name: *const c_char);
    pub fn nemo_file_info_add_string_attribute(file: *mut NemoFileInfo, attribute_name: *const c_char, value: *const c_char);
    pub fn nemo_file_info_can_write(file: *mut NemoFileInfo) -> gboolean;
    pub fn nemo_file_info_get_activation_uri(file: *mut NemoFileInfo) -> *mut c_char;
    pub fn nemo_file_info_get_file_type(file: *mut NemoFileInfo) -> gio::GFileType;
    pub fn nemo_file_info_get_location(file: *mut NemoFileInfo) -> *mut gio::GFile;
    pub fn nemo_file_info_get_mime_type(file: *mut NemoFileInfo) -> *mut c_char;
    pub fn nemo_file_info_get_mount(file: *mut NemoFileInfo) -> *mut gio::GMount;
    pub fn nemo_file_info_get_name(file: *mut NemoFileInfo) -> *mut c_char;
    pub fn nemo_file_info_get_parent_info(file: *mut NemoFileInfo) -> *mut NemoFileInfo;
    pub fn nemo_file_info_get_parent_location(file: *mut NemoFileInfo) -> *mut gio::GFile;
    pub fn nemo_file_info_get_parent_uri(file: *mut NemoFileInfo) -> *mut c_char;
    pub fn nemo_file_info_get_string_attribute(file: *mut NemoFileInfo, attribute_name: *const c_char) -> *mut c_char;
    pub fn nemo_file_info_get_uri(file: *mut NemoFileInfo) -> *mut c_char;
    pub fn nemo_file_info_get_uri_scheme(file: *mut NemoFileInfo) -> *mut c_char;
    pub fn nemo_file_info_invalidate_extension_info(file: *mut NemoFileInfo);
    pub fn nemo_file_info_is_directory(file: *mut NemoFileInfo) -> gboolean;
    pub fn nemo_file_info_is_gone(file: *mut NemoFileInfo) -> gboolean;
    pub fn nemo_file_info_is_mime_type(file: *mut NemoFileInfo, mime_type: *const c_char) -> gboolean;

    //=========================================================================
    // NemoInfoProvider
    //=========================================================================
    pub fn nemo_info_provider_get_type() -> GType;
    pub fn nemo_info_provider_update_complete_invoke(update_complete: *mut gobject::GClosure, provider: *mut NemoInfoProvider, handle: *mut NemoOperationHandle, result: NemoOperationResult);
    pub fn nemo_info_provider_cancel_update(provider: *mut NemoInfoProvider, handle: *mut NemoOperationHandle);
    pub fn nemo_info_provider_update_file_info(provider: *mut NemoInfoProvider, file: *mut NemoFileInfo, update_complete: *mut gobject::GClosure, handle: *mut *mut NemoOperationHandle) -> NemoOperationResult;

    //=========================================================================
    // NemoLocationWidgetProvider
    //=========================================================================
    pub fn nemo_location_widget_provider_get_type() -> GType;
    pub fn nemo_location_widget_provider_get_widget(provider: *mut NemoLocationWidgetProvider, uri: *const c_char, window: *mut gtk::GtkWidget) -> *mut gtk::GtkWidget;

    //=========================================================================
    // NemoMenuProvider
    //=========================================================================
    pub fn nemo_menu_provider_get_type() -> GType;
    pub fn nemo_menu_provider_emit_items_updated_signal(provider: *mut NemoMenuProvider);
    pub fn nemo_menu_provider_get_background_items(provider: *mut NemoMenuProvider, window: *mut gtk::GtkWidget, current_folder: *mut NemoFileInfo) -> *mut glib::GList;
    pub fn nemo_menu_provider_get_file_items(provider: *mut NemoMenuProvider, window: *mut gtk::GtkWidget, files: *mut glib::GList) -> *mut glib::GList;

    //=========================================================================
    // NemoNameAndDescProvider
    //=========================================================================
    pub fn nemo_name_and_desc_provider_get_type() -> GType;
    pub fn nemo_name_and_desc_provider_get_name_and_desc(provider: *mut NemoNameAndDescProvider) -> *mut glib::GList;

    //=========================================================================
    // NemoPropertyPageProvider
    //=========================================================================
    pub fn nemo_property_page_provider_get_type() -> GType;
    pub fn nemo_property_page_provider_get_pages(provider: *mut NemoPropertyPageProvider, files: *mut glib::GList) -> *mut glib::GList;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn nemo_get_resource() -> *mut gio::GResource;
    pub fn nemo_module_initialize(module: *mut gobject::GTypeModule);
    pub fn nemo_module_list_types(types: *mut *const GType, num_types: *mut c_int);
    pub fn nemo_module_shutdown();

}
