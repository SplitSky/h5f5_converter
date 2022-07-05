#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result};

//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
//#[repr(C)]
pub struct Data {
    id: i64,
    datasets: Vec<DataSet>    
}

//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
//#[repr(u8)]
pub struct Info {
    id: i64,
    datasets: Vec<DataSet>,
    meta: String
}


//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
pub struct DataSet {
    form: i64,
    dim: i64,
    data: Vec<u32>
}

impl DataSet {
    pub fn new(&mut self, form: i64, dim: i64, data: Vec<u32>) -> Self {
        Self {form, dim, data}
    }
}


fn read_hdf5() -> Result<()> {
    let file = File::open("nanowire_data.h5")?; // open for reading
    let ds = file.dataset("info/spectrometer_energy")?; // open the dataset
    let ds2 = file.dataset("info/spectrometer_wavelength")?;
    let ds3 = file.dataset("info/tcspc_axis")?;

    Ok(())
}

fn main() -> Result<()> {
    read_hdf5()?;
    Ok(())
}
