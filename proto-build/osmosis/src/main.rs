//#[cfg(feature = "cosmos")]
//use proto_build_osmosis::cosmos;
#[cfg(feature = "osmosis")]
use proto_build_osmosis::osmosis;

fn main() {
    //#[cfg(feature = "cosmos")]
    //cosmos::main();
    #[cfg(feature = "osmosis")]
    osmosis::main();
}
