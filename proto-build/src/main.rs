#[cfg(feature = "cosmos")]
use proto_build::cosmos;
#[cfg(feature = "osmosis")]
use proto_build::osmosis;

fn main() {
    #[cfg(feature = "cosmos")]
    cosmos::main();
    #[cfg(feature = "osmosis")]
    osmosis::main();
}
