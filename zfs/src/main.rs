use libzfs::Libzfs;

fn main() {
    let mut libzfs = Libzfs::new();
    let mypool = libzfs.pool_by_name("pool1");
    let p1 = mypool.unwrap();
    println!("{:#?}, {:#?}, {:#?}", p1.name(), p1.state(), p1.vdev_tree());
}
