pub mod validationsvc {
    tonic::include_proto!("validationsvc");
}

pub mod ticketsrvc {
    tonic::include_proto!("ticketsrvc");
}

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("proto_descriptor");
