#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result, Selection};
use ndarray::{arr2,s};
use serde::{Deserialize, Serialize};
use serde_json::Result;

//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
//#[repr(C)]

#[derive(Serialize, Deserialize)]
pub struct FileNode {
    filename: String,
    groups: Vec<hdf5::Group>
}

impl FileNode {
    pub fn new(&mut self, filename_init: String, groupslist: Vec<hdf5::Group>) -> Self {
        Self {filename: filename_init, groups: groupslist}
    }
}


// end of file struct

//pub struct Group {
//    key: String,
//    datasets: Vec<DataSet>,
//    groups: Vec<Group>
//}

//impl Group {
//    pub fn new(key: String, datasets: Vec<DataSet>, groups: Vec<Group>) -> Self {
//        Self {key, datasets, groups}
//    }
//}

//impl Group {
//    pub fn add_dataset(&mut self, key_in: String, data_in: Vec<u32>, dimension: i64) { // appends a
        // data set to a group
        //let mut dataset_temp = DataSet { id: key_in, dim: dimension, data: data_in};
//        self.datasets.push(DataSet { id: key_in, dim: dimension, data: data_in})

//    }
//}

//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
// Data set struct
//pub struct DataSet {
//    id: String,
//    dim: i64,
//    data: Vec<u32>
//}

//impl DataSet {
//    pub fn new(&mut self, file: &File ,id: String, dim: i64, data: Vec<u32>, position: String) -> Self {
        // obtain data dimensions
        // position is a string allowing the hft5 library in locating the data set
//        let dataSet_temp = file.dataset(position).unwrap();
        // // obtain data

//        Self {id, dim, data}
//    }
//}


fn read_hdf5() -> Result<()> {
    
    let mut b = Vec::new();
    b.push(vec!["nanowire_data.h5"]);
    b.push(vec!["info", "spectrometer_energy", "spectrometer_wavelength"]);
    b.push(vec!["nanowires", "fitting_errors", "geometry", "pic", "power_dependent_tcspc", "spectral_bottom", "spectral_top", "tcspc_bottom", "tcspc_top"]);
    // use this array to decode the format and put it into a struct.
    // Then convert the struct into a JSON format and return
    
    //let mut structure = Vec::new();
    

    //let file = File::open("nanowire_data.h5").unwrap(); // open for reading
    //let ds = file.dataset("info/spectrometer_energy").unwrap(); // open the dataset
    //let ds2 = file.dataset("info/spectrometer_wavelength").unwrap();
    //let ds3 = file.dataset("info/tcspc_axis").unwrap();
    
    //let a = ds.shape(); // returns a vector of the shape of the data set
    //let list = ds.read_2d::<f64>().unwrap();


    //let file = File::open(filename).unwrap(); // open file for reading
    //let keys = file.groups()?; // list groups contained within a file
    

    //for key in keys {
    //    dbg!(key.name());
    //    dbg!(key.member_names()?);
    //}

    //for member in members {
    //    dbg!(member.member_names());
    //    dbg!(member.datasets());
    //    dbg!(member.named_datatypes());
    //}
    



    let filename_init = "nanowires_data.h5";
    let file = File::open(filename_init).unwrap(); // open file for reading
    // created file object
    let node = FileNode { filename: filename_init.to_string(), groups: file.groups().unwrap()};
    
    // convert the FileNode into a JSON file using Serde
    let j = serde_json::to_string(&mut node).unwrap();
     

    Ok(())
}

fn main() -> Result<()> {
    read_hdf5()?;
    Ok(())
}
