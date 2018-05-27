extern crate protobuf;

mod test_default_values_pb;
mod test_group_pb;
mod test_import_nested_pb;
mod test_import_nested_imported_pb;
mod test_import_pkg_nested_pb;
mod test_import_pkg_nested_imported_pb;
mod test_import_root_pb;
mod test_import_root_imported_pb;
mod test_import_nonunique_pb;
mod test_import_nonunique_1_pb;
mod test_import_nonunique_2_pb;
mod test_required_pb;
mod test_sanitize_file_name_pb;
mod test_special_characters_file_name__pb;

mod test_required;
mod test_default_values;

mod test_oneof_default_value_pb;
mod test_oneof_default_value;

mod struct_pb;

// Taken from rust-protobuf 1.0.24 to make sure
// that old generated code it is still compatible with latest rust-protobuf.
mod test_basic_pb_1_0_24;