// This file is generated by rust-protobuf 3.5.0. Do not edit
// .proto file is parsed by protoc --rust_out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `specification.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_0;

// @@protoc_insertion_point(message:Motors)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Motors {
    // message fields
    // @@protoc_insertion_point(field:Motors.left)
    pub left: i32,
    // @@protoc_insertion_point(field:Motors.right)
    pub right: i32,
    // special fields
    // @@protoc_insertion_point(special_field:Motors.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Motors {
    fn default() -> &'a Motors {
        <Motors as ::protobuf::Message>::default_instance()
    }
}

impl Motors {
    pub fn new() -> Motors {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "left",
            |m: &Motors| { &m.left },
            |m: &mut Motors| { &mut m.left },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "right",
            |m: &Motors| { &m.right },
            |m: &mut Motors| { &mut m.right },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Motors>(
            "Motors",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Motors {
    const NAME: &'static str = "Motors";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.left = is.read_int32()?;
                },
                16 => {
                    self.right = is.read_int32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.left != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.left);
        }
        if self.right != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.right);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.left != 0 {
            os.write_int32(1, self.left)?;
        }
        if self.right != 0 {
            os.write_int32(2, self.right)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Motors {
        Motors::new()
    }

    fn clear(&mut self) {
        self.left = 0;
        self.right = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Motors {
        static instance: Motors = Motors {
            left: 0,
            right: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Motors {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Motors").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Motors {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Motors {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:SharedData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SharedData {
    // message fields
    // @@protoc_insertion_point(field:SharedData.motors)
    pub motors: ::protobuf::MessageField<Motors>,
    // special fields
    // @@protoc_insertion_point(special_field:SharedData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SharedData {
    fn default() -> &'a SharedData {
        <SharedData as ::protobuf::Message>::default_instance()
    }
}

impl SharedData {
    pub fn new() -> SharedData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Motors>(
            "motors",
            |m: &SharedData| { &m.motors },
            |m: &mut SharedData| { &mut m.motors },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SharedData>(
            "SharedData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SharedData {
    const NAME: &'static str = "SharedData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.motors)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.motors.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.motors.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SharedData {
        SharedData::new()
    }

    fn clear(&mut self) {
        self.motors.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SharedData {
        static instance: SharedData = SharedData {
            motors: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SharedData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SharedData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SharedData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SharedData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13specification.proto\"2\n\x06Motors\x12\x12\n\x04left\x18\x01\x20\
    \x01(\x05R\x04left\x12\x14\n\x05right\x18\x02\x20\x01(\x05R\x05right\"-\
    \n\nSharedData\x12\x1f\n\x06motors\x18\x01\x20\x01(\x0b2\x07.MotorsR\x06\
    motorsJ\xe7\x01\n\x06\x12\x04\0\0\t\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\
    \n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\
    \x08\x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x02\x11\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x03\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\
    \x08\x0c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x0f\x10\n\x0b\n\x04\x04\
    \0\x02\x01\x12\x03\x04\x02\x12\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x08\r\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x04\x10\x11\n\n\n\x02\x04\x01\x12\x04\x07\0\t\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x08\x02\x14\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x08\x02\x08\n\x0c\
    \n\x05\x04\x01\x02\0\x01\x12\x03\x08\t\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03\x08\x12\x13b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Motors::generated_message_descriptor_data());
            messages.push(SharedData::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
