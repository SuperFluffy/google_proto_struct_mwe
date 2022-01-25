#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    #[prost(map = "string, message", tag = "1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        #[prost(double, tag = "2")]
        NumberValue(f64),
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        #[prost(message, tag = "5")]
        StructValue(super::Struct),
        #[prost(message, tag = "6")]
        ListValue(super::ListValue),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    NullValue = 0,
}
