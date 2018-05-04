// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702



use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct TensorProto {
    // message fields
    pub dtype: super::types::DataType,
    pub tensor_shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    pub version_number: i32,
    pub tensor_content: ::std::vec::Vec<u8>,
    pub half_val: ::std::vec::Vec<i32>,
    pub float_val: ::std::vec::Vec<f32>,
    pub double_val: ::std::vec::Vec<f64>,
    pub int_val: ::std::vec::Vec<i32>,
    pub string_val: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub scomplex_val: ::std::vec::Vec<f32>,
    pub int64_val: ::std::vec::Vec<i64>,
    pub bool_val: ::std::vec::Vec<bool>,
    pub dcomplex_val: ::std::vec::Vec<f64>,
    pub resource_handle_val: ::protobuf::RepeatedField<super::resource_handle::ResourceHandle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorProto {}

impl TensorProto {
    pub fn new() -> TensorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorProto {
        static mut instance: ::protobuf::lazy::Lazy<TensorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorProto,
        };
        unsafe {
            instance.get(TensorProto::new)
        }
    }

    // .tensorflow.DataType dtype = 1;

    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }

    fn get_dtype_for_reflect(&self) -> &super::types::DataType {
        &self.dtype
    }

    fn mut_dtype_for_reflect(&mut self) -> &mut super::types::DataType {
        &mut self.dtype
    }

    // .tensorflow.TensorShapeProto tensor_shape = 2;

    pub fn clear_tensor_shape(&mut self) {
        self.tensor_shape.clear();
    }

    pub fn has_tensor_shape(&self) -> bool {
        self.tensor_shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.tensor_shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.tensor_shape.is_none() {
            self.tensor_shape.set_default();
        }
        self.tensor_shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.tensor_shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    pub fn get_tensor_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.tensor_shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }

    fn get_tensor_shape_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &self.tensor_shape
    }

    fn mut_tensor_shape_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &mut self.tensor_shape
    }

    // int32 version_number = 3;

    pub fn clear_version_number(&mut self) {
        self.version_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_version_number(&mut self, v: i32) {
        self.version_number = v;
    }

    pub fn get_version_number(&self) -> i32 {
        self.version_number
    }

    fn get_version_number_for_reflect(&self) -> &i32 {
        &self.version_number
    }

    fn mut_version_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.version_number
    }

    // bytes tensor_content = 4;

    pub fn clear_tensor_content(&mut self) {
        self.tensor_content.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensor_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.tensor_content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tensor_content
    }

    // Take field
    pub fn take_tensor_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tensor_content, ::std::vec::Vec::new())
    }

    pub fn get_tensor_content(&self) -> &[u8] {
        &self.tensor_content
    }

    fn get_tensor_content_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tensor_content
    }

    fn mut_tensor_content_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tensor_content
    }

    // repeated int32 half_val = 13;

    pub fn clear_half_val(&mut self) {
        self.half_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_half_val(&mut self, v: ::std::vec::Vec<i32>) {
        self.half_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_half_val(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.half_val
    }

    // Take field
    pub fn take_half_val(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.half_val, ::std::vec::Vec::new())
    }

    pub fn get_half_val(&self) -> &[i32] {
        &self.half_val
    }

    fn get_half_val_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.half_val
    }

    fn mut_half_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.half_val
    }

    // repeated float float_val = 5;

    pub fn clear_float_val(&mut self) {
        self.float_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_float_val(&mut self, v: ::std::vec::Vec<f32>) {
        self.float_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float_val(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_val
    }

    // Take field
    pub fn take_float_val(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float_val, ::std::vec::Vec::new())
    }

    pub fn get_float_val(&self) -> &[f32] {
        &self.float_val
    }

    fn get_float_val_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.float_val
    }

    fn mut_float_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_val
    }

    // repeated double double_val = 6;

    pub fn clear_double_val(&mut self) {
        self.double_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_double_val(&mut self, v: ::std::vec::Vec<f64>) {
        self.double_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double_val(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_val
    }

    // Take field
    pub fn take_double_val(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double_val, ::std::vec::Vec::new())
    }

    pub fn get_double_val(&self) -> &[f64] {
        &self.double_val
    }

    fn get_double_val_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.double_val
    }

    fn mut_double_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_val
    }

    // repeated int32 int_val = 7;

    pub fn clear_int_val(&mut self) {
        self.int_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_val(&mut self, v: ::std::vec::Vec<i32>) {
        self.int_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int_val(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int_val
    }

    // Take field
    pub fn take_int_val(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int_val, ::std::vec::Vec::new())
    }

    pub fn get_int_val(&self) -> &[i32] {
        &self.int_val
    }

    fn get_int_val_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.int_val
    }

    fn mut_int_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int_val
    }

    // repeated bytes string_val = 8;

    pub fn clear_string_val(&mut self) {
        self.string_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_val(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.string_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_val(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.string_val
    }

    // Take field
    pub fn take_string_val(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.string_val, ::protobuf::RepeatedField::new())
    }

    pub fn get_string_val(&self) -> &[::std::vec::Vec<u8>] {
        &self.string_val
    }

    fn get_string_val_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.string_val
    }

    fn mut_string_val_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.string_val
    }

    // repeated float scomplex_val = 9;

    pub fn clear_scomplex_val(&mut self) {
        self.scomplex_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_scomplex_val(&mut self, v: ::std::vec::Vec<f32>) {
        self.scomplex_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scomplex_val(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.scomplex_val
    }

    // Take field
    pub fn take_scomplex_val(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.scomplex_val, ::std::vec::Vec::new())
    }

    pub fn get_scomplex_val(&self) -> &[f32] {
        &self.scomplex_val
    }

    fn get_scomplex_val_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.scomplex_val
    }

    fn mut_scomplex_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.scomplex_val
    }

    // repeated int64 int64_val = 10;

    pub fn clear_int64_val(&mut self) {
        self.int64_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64_val(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64_val(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_val
    }

    // Take field
    pub fn take_int64_val(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64_val, ::std::vec::Vec::new())
    }

    pub fn get_int64_val(&self) -> &[i64] {
        &self.int64_val
    }

    fn get_int64_val_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.int64_val
    }

    fn mut_int64_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_val
    }

    // repeated bool bool_val = 11;

    pub fn clear_bool_val(&mut self) {
        self.bool_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool_val(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool_val(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_val
    }

    // Take field
    pub fn take_bool_val(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool_val, ::std::vec::Vec::new())
    }

    pub fn get_bool_val(&self) -> &[bool] {
        &self.bool_val
    }

    fn get_bool_val_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.bool_val
    }

    fn mut_bool_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_val
    }

    // repeated double dcomplex_val = 12;

    pub fn clear_dcomplex_val(&mut self) {
        self.dcomplex_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_dcomplex_val(&mut self, v: ::std::vec::Vec<f64>) {
        self.dcomplex_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dcomplex_val(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.dcomplex_val
    }

    // Take field
    pub fn take_dcomplex_val(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.dcomplex_val, ::std::vec::Vec::new())
    }

    pub fn get_dcomplex_val(&self) -> &[f64] {
        &self.dcomplex_val
    }

    fn get_dcomplex_val_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.dcomplex_val
    }

    fn mut_dcomplex_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.dcomplex_val
    }

    // repeated .tensorflow.ResourceHandle resource_handle_val = 14;

    pub fn clear_resource_handle_val(&mut self) {
        self.resource_handle_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_handle_val(&mut self, v: ::protobuf::RepeatedField<super::resource_handle::ResourceHandle>) {
        self.resource_handle_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resource_handle_val(&mut self) -> &mut ::protobuf::RepeatedField<super::resource_handle::ResourceHandle> {
        &mut self.resource_handle_val
    }

    // Take field
    pub fn take_resource_handle_val(&mut self) -> ::protobuf::RepeatedField<super::resource_handle::ResourceHandle> {
        ::std::mem::replace(&mut self.resource_handle_val, ::protobuf::RepeatedField::new())
    }

    pub fn get_resource_handle_val(&self) -> &[super::resource_handle::ResourceHandle] {
        &self.resource_handle_val
    }

    fn get_resource_handle_val_for_reflect(&self) -> &::protobuf::RepeatedField<super::resource_handle::ResourceHandle> {
        &self.resource_handle_val
    }

    fn mut_resource_handle_val_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::resource_handle::ResourceHandle> {
        &mut self.resource_handle_val
    }
}

impl ::protobuf::Message for TensorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor_shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource_handle_val {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.dtype, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor_shape)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version_number = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tensor_content)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.half_val)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float_val)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double_val)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.int_val)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.string_val)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.scomplex_val)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.int64_val)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool_val)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.dcomplex_val)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resource_handle_val)?;
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
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.dtype);
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.version_number != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.tensor_content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.tensor_content);
        }
        if !self.half_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(13, &self.half_val);
        }
        if !self.float_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.float_val.len() * 4) as u32) + (self.float_val.len() * 4) as u32;
        }
        if !self.double_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.double_val.len() * 8) as u32) + (self.double_val.len() * 8) as u32;
        }
        if !self.int_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.int_val);
        }
        for value in &self.string_val {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        if !self.scomplex_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.scomplex_val.len() * 4) as u32) + (self.scomplex_val.len() * 4) as u32;
        }
        if !self.int64_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(10, &self.int64_val);
        }
        if !self.bool_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.bool_val.len() * 1) as u32) + (self.bool_val.len() * 1) as u32;
        }
        if !self.dcomplex_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size((self.dcomplex_val.len() * 8) as u32) + (self.dcomplex_val.len() * 8) as u32;
        }
        for value in &self.resource_handle_val {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(1, self.dtype.value())?;
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.version_number != 0 {
            os.write_int32(3, self.version_number)?;
        }
        if !self.tensor_content.is_empty() {
            os.write_bytes(4, &self.tensor_content)?;
        }
        if !self.half_val.is_empty() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.half_val))?;
            for v in &self.half_val {
                os.write_int32_no_tag(*v)?;
            };
        }
        if !self.float_val.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.float_val.len() * 4) as u32)?;
            for v in &self.float_val {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.double_val.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.double_val.len() * 8) as u32)?;
            for v in &self.double_val {
                os.write_double_no_tag(*v)?;
            };
        }
        if !self.int_val.is_empty() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int_val))?;
            for v in &self.int_val {
                os.write_int32_no_tag(*v)?;
            };
        }
        for v in &self.string_val {
            os.write_bytes(8, &v)?;
        };
        if !self.scomplex_val.is_empty() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.scomplex_val.len() * 4) as u32)?;
            for v in &self.scomplex_val {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.int64_val.is_empty() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int64_val))?;
            for v in &self.int64_val {
                os.write_int64_no_tag(*v)?;
            };
        }
        if !self.bool_val.is_empty() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.bool_val.len() * 1) as u32)?;
            for v in &self.bool_val {
                os.write_bool_no_tag(*v)?;
            };
        }
        if !self.dcomplex_val.is_empty() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.dcomplex_val.len() * 8) as u32)?;
            for v in &self.dcomplex_val {
                os.write_double_no_tag(*v)?;
            };
        }
        for v in &self.resource_handle_val {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for TensorProto {
    fn new() -> TensorProto {
        TensorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    TensorProto::get_dtype_for_reflect,
                    TensorProto::mut_dtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "tensor_shape",
                    TensorProto::get_tensor_shape_for_reflect,
                    TensorProto::mut_tensor_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version_number",
                    TensorProto::get_version_number_for_reflect,
                    TensorProto::mut_version_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tensor_content",
                    TensorProto::get_tensor_content_for_reflect,
                    TensorProto::mut_tensor_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "half_val",
                    TensorProto::get_half_val_for_reflect,
                    TensorProto::mut_half_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_val",
                    TensorProto::get_float_val_for_reflect,
                    TensorProto::mut_float_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_val",
                    TensorProto::get_double_val_for_reflect,
                    TensorProto::mut_double_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int_val",
                    TensorProto::get_int_val_for_reflect,
                    TensorProto::mut_int_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_val",
                    TensorProto::get_string_val_for_reflect,
                    TensorProto::mut_string_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scomplex_val",
                    TensorProto::get_scomplex_val_for_reflect,
                    TensorProto::mut_scomplex_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "int64_val",
                    TensorProto::get_int64_val_for_reflect,
                    TensorProto::mut_int64_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_val",
                    TensorProto::get_bool_val_for_reflect,
                    TensorProto::mut_bool_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "dcomplex_val",
                    TensorProto::get_dcomplex_val_for_reflect,
                    TensorProto::mut_dcomplex_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::resource_handle::ResourceHandle>>(
                    "resource_handle_val",
                    TensorProto::get_resource_handle_val_for_reflect,
                    TensorProto::mut_resource_handle_val_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorProto>(
                    "TensorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorProto {
    fn clear(&mut self) {
        self.clear_dtype();
        self.clear_tensor_shape();
        self.clear_version_number();
        self.clear_tensor_content();
        self.clear_half_val();
        self.clear_float_val();
        self.clear_double_val();
        self.clear_int_val();
        self.clear_string_val();
        self.clear_scomplex_val();
        self.clear_int64_val();
        self.clear_bool_val();
        self.clear_dcomplex_val();
        self.clear_resource_handle_val();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&tensorflow/core/framework/tensor.proto\x12\ntensorflow\x1a/tensorflow\
    /core/framework/resource_handle.proto\x1a,tensorflow/core/framework/tens\
    or_shape.proto\x1a%tensorflow/core/framework/types.proto\"\xc1\x04\n\x0b\
    TensorProto\x12*\n\x05dtype\x18\x01\x20\x01(\x0e2\x14.tensorflow.DataTyp\
    eR\x05dtype\x12?\n\x0ctensor_shape\x18\x02\x20\x01(\x0b2\x1c.tensorflow.\
    TensorShapeProtoR\x0btensorShape\x12%\n\x0eversion_number\x18\x03\x20\
    \x01(\x05R\rversionNumber\x12%\n\x0etensor_content\x18\x04\x20\x01(\x0cR\
    \rtensorContent\x12\x1d\n\x08half_val\x18\r\x20\x03(\x05R\x07halfValB\
    \x02\x10\x01\x12\x1f\n\tfloat_val\x18\x05\x20\x03(\x02R\x08floatValB\x02\
    \x10\x01\x12!\n\ndouble_val\x18\x06\x20\x03(\x01R\tdoubleValB\x02\x10\
    \x01\x12\x1b\n\x07int_val\x18\x07\x20\x03(\x05R\x06intValB\x02\x10\x01\
    \x12\x1d\n\nstring_val\x18\x08\x20\x03(\x0cR\tstringVal\x12%\n\x0cscompl\
    ex_val\x18\t\x20\x03(\x02R\x0bscomplexValB\x02\x10\x01\x12\x1f\n\tint64_\
    val\x18\n\x20\x03(\x03R\x08int64ValB\x02\x10\x01\x12\x1d\n\x08bool_val\
    \x18\x0b\x20\x03(\x08R\x07boolValB\x02\x10\x01\x12%\n\x0cdcomplex_val\
    \x18\x0c\x20\x03(\x01R\x0bdcomplexValB\x02\x10\x01\x12J\n\x13resource_ha\
    ndle_val\x18\x0e\x20\x03(\x0b2\x1a.tensorflow.ResourceHandleR\x11resourc\
    eHandleValB-\n\x18org.tensorflow.frameworkB\x0cTensorProtosP\x01\xf8\x01\
    \x01b\x06proto3\
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
