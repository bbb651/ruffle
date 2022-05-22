//! `flash.ui.Mouse` builtin

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};

fn instance_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Err("The Mouse class cannot be constructed.".into())
}

fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

fn cursor<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    log::warn!("Mouse.cursor: not yet implemented");
    Ok(Value::Undefined)
}

fn set_cursor<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    log::warn!("Mouse.cursor: not yet implemented");
    Ok(Value::Undefined)
}

fn supports_cursor<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Bool(false))
}

fn supports_native_cursor<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Bool(false))
}

fn hide<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    activation.context.ui.set_mouse_visible(false);
    Ok(Value::Undefined)
}

fn register_cursor<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    log::warn!("Mouse.registerCursor: not yet implemented");
    Ok(Value::Undefined)
}

fn show<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    activation.context.ui.set_mouse_visible(true);
    Ok(Value::Undefined)
}

fn unregister_cursor<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    log::warn!("Mouse.unregisterCursor: not yet implemented");
    Ok(Value::Undefined)
}

pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.ui"), "Mouse"),
        Some(QName::new(Namespace::package(""), "Object").into()),
        Method::from_builtin(instance_init, "<Mouse instance initializer>", mc),
        Method::from_builtin(class_init, "<Mouse class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED | ClassAttributes::FINAL);

    const PUBLIC_CLASS_PROPERTIES: &[(&str, Option<NativeMethodImpl>, Option<NativeMethodImpl>)] =
        &[
            ("cursor", Some(cursor), Some(set_cursor)),
            ("supportsCursor", Some(supports_cursor), None),
            ("supportsNativeCursor", Some(supports_native_cursor), None),
        ];
    write.define_public_builtin_class_properties(mc, PUBLIC_CLASS_PROPERTIES);

    const PUBLIC_CLASS_METHODS: &[(&str, NativeMethodImpl)] = &[
        ("show", show),
        ("registerCursor", register_cursor),
        ("hide", hide),
        ("unregisterCursor", unregister_cursor),
    ];
    write.define_public_builtin_class_methods(mc, PUBLIC_CLASS_METHODS);

    class
}
