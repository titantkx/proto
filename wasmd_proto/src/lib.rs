use cosmos_sdk_proto::cosmos;

pub mod cosmwasm {
    pub mod wasm {
        pub mod v1 {
            include!("prost/cosmwasm.wasm.v1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/cosmwasm.wasm.v1.tonic.rs");
        }
    }
}
