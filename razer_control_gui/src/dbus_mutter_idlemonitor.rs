// This code was autogenerated with `dbus-codegen-rust -d org.gnome.Mutter.IdleMonitor -p /org/gnome/Mutter/IdleMonitor/Core -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0, ))
    }

    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgGnomeMutterIdleMonitor {
    fn get_idletime(&self) -> Result<u64, dbus::Error>;
    fn add_idle_watch(&self, interval: u64) -> Result<u32, dbus::Error>;
    fn add_user_active_watch(&self) -> Result<u32, dbus::Error>;
    fn remove_watch(&self, id: u32) -> Result<(), dbus::Error>;
    fn reset_idletime(&self) -> Result<(), dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgGnomeMutterIdleMonitor for blocking::Proxy<'a, C> {

    fn get_idletime(&self) -> Result<u64, dbus::Error> {
        self.method_call("org.gnome.Mutter.IdleMonitor", "GetIdletime", ())
            .and_then(|r: (u64, )| Ok(r.0, ))
    }

    fn add_idle_watch(&self, interval: u64) -> Result<u32, dbus::Error> {
        self.method_call("org.gnome.Mutter.IdleMonitor", "AddIdleWatch", (interval, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn add_user_active_watch(&self) -> Result<u32, dbus::Error> {
        self.method_call("org.gnome.Mutter.IdleMonitor", "AddUserActiveWatch", ())
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn remove_watch(&self, id: u32) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.Mutter.IdleMonitor", "RemoveWatch", (id, ))
    }

    fn reset_idletime(&self) -> Result<(), dbus::Error> {
        self.method_call("org.gnome.Mutter.IdleMonitor", "ResetIdletime", ())
    }
}

#[derive(Debug)]
pub struct OrgGnomeMutterIdleMonitorWatchFired {
    pub id: u32,
}

impl arg::AppendAll for OrgGnomeMutterIdleMonitorWatchFired {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
    }
}

impl arg::ReadAll for OrgGnomeMutterIdleMonitorWatchFired {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgGnomeMutterIdleMonitorWatchFired {
            id: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgGnomeMutterIdleMonitorWatchFired {
    const NAME: &'static str = "WatchFired";
    const INTERFACE: &'static str = "org.gnome.Mutter.IdleMonitor";
}