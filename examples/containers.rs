extern crate env_logger;
extern crate shiplift;

use shiplift::Docker;

fn main() {
    env_logger::init().unwrap();
    let docker = Docker::new().unwrap();
    for c in docker
        .containers()
        .list(&Default::default())
        .unwrap()
    {
        println!("container -> {:?}", c)
    }
}
