// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702



use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct FunctionDefLibrary {
    // message fields
    pub function: ::protobuf::RepeatedField<FunctionDef>,
    pub gradient: ::protobuf::RepeatedField<GradientDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FunctionDefLibrary {}

impl FunctionDefLibrary {
    pub fn new() -> FunctionDefLibrary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FunctionDefLibrary {
        static mut instance: ::protobuf::lazy::Lazy<FunctionDefLibrary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FunctionDefLibrary,
        };
        unsafe {
            instance.get(FunctionDefLibrary::new)
        }
    }

    // repeated .tensorflow.FunctionDef function = 1;

    pub fn clear_function(&mut self) {
        self.function.clear();
    }

    // Param is passed by value, moved
    pub fn set_function(&mut self, v: ::protobuf::RepeatedField<FunctionDef>) {
        self.function = v;
    }

    // Mutable pointer to the field.
    pub fn mut_function(&mut self) -> &mut ::protobuf::RepeatedField<FunctionDef> {
        &mut self.function
    }

    // Take field
    pub fn take_function(&mut self) -> ::protobuf::RepeatedField<FunctionDef> {
        ::std::mem::replace(&mut self.function, ::protobuf::RepeatedField::new())
    }

    pub fn get_function(&self) -> &[FunctionDef] {
        &self.function
    }

    fn get_function_for_reflect(&self) -> &::protobuf::RepeatedField<FunctionDef> {
        &self.function
    }

    fn mut_function_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FunctionDef> {
        &mut self.function
    }

    // repeated .tensorflow.GradientDef gradient = 2;

    pub fn clear_gradient(&mut self) {
        self.gradient.clear();
    }

    // Param is passed by value, moved
    pub fn set_gradient(&mut self, v: ::protobuf::RepeatedField<GradientDef>) {
        self.gradient = v;
    }

    // Mutable pointer to the field.
    pub fn mut_gradient(&mut self) -> &mut ::protobuf::RepeatedField<GradientDef> {
        &mut self.gradient
    }

    // Take field
    pub fn take_gradient(&mut self) -> ::protobuf::RepeatedField<GradientDef> {
        ::std::mem::replace(&mut self.gradient, ::protobuf::RepeatedField::new())
    }

    pub fn get_gradient(&self) -> &[GradientDef] {
        &self.gradient
    }

    fn get_gradient_for_reflect(&self) -> &::protobuf::RepeatedField<GradientDef> {
        &self.gradient
    }

    fn mut_gradient_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GradientDef> {
        &mut self.gradient
    }
}

impl ::protobuf::Message for FunctionDefLibrary {
    fn is_initialized(&self) -> bool {
        for v in &self.function {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.gradient {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.function)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.gradient)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.function {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.gradient {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.function {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.gradient {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FunctionDefLibrary {
    fn new() -> FunctionDefLibrary {
        FunctionDefLibrary::new()
    }

    fn descriptor_static(_: ::std::option::Option<FunctionDefLibrary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FunctionDef>>(
                    "function",
                    FunctionDefLibrary::get_function_for_reflect,
                    FunctionDefLibrary::mut_function_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GradientDef>>(
                    "gradient",
                    FunctionDefLibrary::get_gradient_for_reflect,
                    FunctionDefLibrary::mut_gradient_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FunctionDefLibrary>(
                    "FunctionDefLibrary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FunctionDefLibrary {
    fn clear(&mut self) {
        self.clear_function();
        self.clear_gradient();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FunctionDefLibrary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FunctionDefLibrary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FunctionDef {
    // message fields
    pub signature: ::protobuf::SingularPtrField<super::op_def::OpDef>,
    pub attr: ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue>,
    pub node_def: ::protobuf::RepeatedField<super::node_def::NodeDef>,
    pub ret: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FunctionDef {}

impl FunctionDef {
    pub fn new() -> FunctionDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FunctionDef {
        static mut instance: ::protobuf::lazy::Lazy<FunctionDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FunctionDef,
        };
        unsafe {
            instance.get(FunctionDef::new)
        }
    }

    // .tensorflow.OpDef signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: super::op_def::OpDef) {
        self.signature = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut super::op_def::OpDef {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> super::op_def::OpDef {
        self.signature.take().unwrap_or_else(|| super::op_def::OpDef::new())
    }

    pub fn get_signature(&self) -> &super::op_def::OpDef {
        self.signature.as_ref().unwrap_or_else(|| super::op_def::OpDef::default_instance())
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularPtrField<super::op_def::OpDef> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::op_def::OpDef> {
        &mut self.signature
    }

    // repeated .tensorflow.FunctionDef.AttrEntry attr = 5;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue>) {
        self.attr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &mut self.attr
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        ::std::mem::replace(&mut self.attr, ::std::collections::HashMap::new())
    }

    pub fn get_attr(&self) -> &::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &self.attr
    }

    fn get_attr_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &self.attr
    }

    fn mut_attr_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &mut self.attr
    }

    // repeated .tensorflow.NodeDef node_def = 3;

    pub fn clear_node_def(&mut self) {
        self.node_def.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_def(&mut self, v: ::protobuf::RepeatedField<super::node_def::NodeDef>) {
        self.node_def = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_def(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node_def
    }

    // Take field
    pub fn take_node_def(&mut self) -> ::protobuf::RepeatedField<super::node_def::NodeDef> {
        ::std::mem::replace(&mut self.node_def, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_def(&self) -> &[super::node_def::NodeDef] {
        &self.node_def
    }

    fn get_node_def_for_reflect(&self) -> &::protobuf::RepeatedField<super::node_def::NodeDef> {
        &self.node_def
    }

    fn mut_node_def_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node_def
    }

    // repeated .tensorflow.FunctionDef.RetEntry ret = 4;

    pub fn clear_ret(&mut self) {
        self.ret.clear();
    }

    // Param is passed by value, moved
    pub fn set_ret(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.ret = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ret(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.ret
    }

    // Take field
    pub fn take_ret(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.ret, ::std::collections::HashMap::new())
    }

    pub fn get_ret(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.ret
    }

    fn get_ret_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.ret
    }

    fn mut_ret_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.ret
    }
}

impl ::protobuf::Message for FunctionDef {
    fn is_initialized(&self) -> bool {
        for v in &self.signature {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.node_def {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signature)?;
                },
                5 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(wire_type, is, &mut self.attr)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_def)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.ret)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.signature.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(5, &self.attr);
        for value in &self.node_def {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.ret);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.signature.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(5, &self.attr, os)?;
        for v in &self.node_def {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.ret, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FunctionDef {
    fn new() -> FunctionDef {
        FunctionDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<FunctionDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::op_def::OpDef>>(
                    "signature",
                    FunctionDef::get_signature_for_reflect,
                    FunctionDef::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "attr",
                    FunctionDef::get_attr_for_reflect,
                    FunctionDef::mut_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::node_def::NodeDef>>(
                    "node_def",
                    FunctionDef::get_node_def_for_reflect,
                    FunctionDef::mut_node_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "ret",
                    FunctionDef::get_ret_for_reflect,
                    FunctionDef::mut_ret_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FunctionDef>(
                    "FunctionDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FunctionDef {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_attr();
        self.clear_node_def();
        self.clear_ret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FunctionDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FunctionDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GradientDef {
    // message fields
    pub function_name: ::std::string::String,
    pub gradient_func: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GradientDef {}

impl GradientDef {
    pub fn new() -> GradientDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GradientDef {
        static mut instance: ::protobuf::lazy::Lazy<GradientDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GradientDef,
        };
        unsafe {
            instance.get(GradientDef::new)
        }
    }

    // string function_name = 1;

    pub fn clear_function_name(&mut self) {
        self.function_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_function_name(&mut self, v: ::std::string::String) {
        self.function_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_function_name(&mut self) -> &mut ::std::string::String {
        &mut self.function_name
    }

    // Take field
    pub fn take_function_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.function_name, ::std::string::String::new())
    }

    pub fn get_function_name(&self) -> &str {
        &self.function_name
    }

    fn get_function_name_for_reflect(&self) -> &::std::string::String {
        &self.function_name
    }

    fn mut_function_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.function_name
    }

    // string gradient_func = 2;

    pub fn clear_gradient_func(&mut self) {
        self.gradient_func.clear();
    }

    // Param is passed by value, moved
    pub fn set_gradient_func(&mut self, v: ::std::string::String) {
        self.gradient_func = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gradient_func(&mut self) -> &mut ::std::string::String {
        &mut self.gradient_func
    }

    // Take field
    pub fn take_gradient_func(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gradient_func, ::std::string::String::new())
    }

    pub fn get_gradient_func(&self) -> &str {
        &self.gradient_func
    }

    fn get_gradient_func_for_reflect(&self) -> &::std::string::String {
        &self.gradient_func
    }

    fn mut_gradient_func_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.gradient_func
    }
}

impl ::protobuf::Message for GradientDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.function_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gradient_func)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.function_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.function_name);
        }
        if !self.gradient_func.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.gradient_func);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.function_name.is_empty() {
            os.write_string(1, &self.function_name)?;
        }
        if !self.gradient_func.is_empty() {
            os.write_string(2, &self.gradient_func)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GradientDef {
    fn new() -> GradientDef {
        GradientDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<GradientDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "function_name",
                    GradientDef::get_function_name_for_reflect,
                    GradientDef::mut_function_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gradient_func",
                    GradientDef::get_gradient_func_for_reflect,
                    GradientDef::mut_gradient_func_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GradientDef>(
                    "GradientDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GradientDef {
    fn clear(&mut self) {
        self.clear_function_name();
        self.clear_gradient_func();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GradientDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GradientDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow/core/framework/function.proto\x12\ntensorflow\x1a*tensorfl\
    ow/core/framework/attr_value.proto\x1a(tensorflow/core/framework/node_de\
    f.proto\x1a&tensorflow/core/framework/op_def.proto\"~\n\x12FunctionDefLi\
    brary\x123\n\x08function\x18\x01\x20\x03(\x0b2\x17.tensorflow.FunctionDe\
    fR\x08function\x123\n\x08gradient\x18\x02\x20\x03(\x0b2\x17.tensorflow.G\
    radientDefR\x08gradient\"\xe1\x02\n\x0bFunctionDef\x12/\n\tsignature\x18\
    \x01\x20\x01(\x0b2\x11.tensorflow.OpDefR\tsignature\x125\n\x04attr\x18\
    \x05\x20\x03(\x0b2!.tensorflow.FunctionDef.AttrEntryR\x04attr\x12.\n\x08\
    node_def\x18\x03\x20\x03(\x0b2\x13.tensorflow.NodeDefR\x07nodeDef\x122\n\
    \x03ret\x18\x04\x20\x03(\x0b2\x20.tensorflow.FunctionDef.RetEntryR\x03re\
    t\x1aN\n\tAttrEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12+\n\
    \x05value\x18\x02\x20\x01(\x0b2\x15.tensorflow.AttrValueR\x05value:\x028\
    \x01\x1a6\n\x08RetEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"W\n\x0bGradientD\
    ef\x12#\n\rfunction_name\x18\x01\x20\x01(\tR\x0cfunctionName\x12#\n\rgra\
    dient_func\x18\x02\x20\x01(\tR\x0cgradientFuncB/\n\x18org.tensorflow.fra\
    meworkB\x0eFunctionProtosP\x01\xf8\x01\x01b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
