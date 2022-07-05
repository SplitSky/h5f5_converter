// imported serde and hdf5
use hdf5::{File, H5Type, Result};

pub struct MainFile{
    info: Vec<Info>,
    data: Vec<Data>,
    meta: u32
}

pub struct Info { // struct for capturing and holdig the data for it to be converted
    node_id: u32,
    node_data: Vec<u32>,
    node_meta: Vec<u32>,
}

pub struct Data {
    node_id: u32,
    node_data: Vec<u32>,
    node_meta: Vec<u32>,
    node_groups: Vec<Dataset>,
}

pub struct Dataset {
    node_id: u32,
    node_data: Vec<u32>,
    node_meta: Vec<u32>,
}

impl Dataset {
    pub fn thing(&mut self) {
        let mut a = Vec::new(); // assigns new vector 
        a.push(1);
    }
}

pub fn create_h5_file() { // create a simple h5 file as a template to test reading function
    // File structure
    // file = nanowires_data.h5
    // info group
    //  1. spectrometer_energy
    //  2. spectrometer_wavelegnth
    //  3. tcspc_axis
    // nanowires group
    //  1. fitting_errors
    //  2. geometry
    //  3. pic
    //  4. power_dependent_tcspc
    //  5. spectral_bottom
    //  6. spectral_top
    //  7. tcspc_bottom
    //  8. tcspc_top

}

pub fn read_h5_file() {

    let file = File::open("nanotubes_data.h5"); // open for reading
    
}

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(u8)]pub enum Color {
    R = 1,
    G = 2,
    B = 3,
}

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(C)]
pub struct Pixel {
    xy: (i64, i64),
    color: Color,
}
use ndarray::{arr2, s};
impl Pixel {
    pub fn new(x: i64, y: i64, color: Color) -> Self {
        Self { xy: (x, y), color }
    }
}
fn read_hdf5() -> Result<()> {
    use Color::*;
    let file = File::open("pixels.h5")?; // open for reading
    let ds = file.dataset("dir/pixels")?; // open the dataset
    assert_eq!(
        // read a slice of the 2-D dataset and verify it
        ds.read_slice::<Pixel, _, _>(s![1.., ..])?,
        arr2(&[
            [Pixel::new(3, 4, G), Pixel::new(4, 5, R)],
            [Pixel::new(5, 6, B), Pixel::new(6, 7, G)],
        ])
    );
    let attr = ds.attr("colors")?; // open the attribute
    assert_eq!(attr.read_1d::<Color>()?.as_slice().unwrap(), &[R, G, B]);
    Ok(())
}
fn main() {
    

}

