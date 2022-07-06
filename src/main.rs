#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result, Selection};
use ndarray::{arr2,s, Dimension};
use serde::{Deserialize, Serialize};
use serde_json;

//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
//#[repr(C)]

//#[derive(H5Type, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FileNodeStruct {
    filename: String,
    groups: Vec<GroupStruct>
}

impl FileNodeStruct {
    pub fn new(filename_init: String, file_init: &hdf5::File) -> Self {
        let mut file = file_init;
        let filename = filename_init;
        let mut groups = Vec::new();
        let mut group_list = file.groups().unwrap();
        
        for entry in group_list { // should include error validation
            groups.push(GroupStruct::new(entry.name(), entry.groups().unwrap(), entry.datasets().unwrap()))
        }
        Self {filename, groups}
    }
}


// end of file struct

pub struct GroupStruct {
    key: String,
    groups: Vec<GroupStruct>,

    // all available possible data sets
    data_set1D: Vec<DataSetStruct1D>,
    data_set2D: Vec<DataSetStruct2D>,
    data_set3D: Vec<DataSetStruct3D>,

}

impl GroupStruct {

    pub fn new(key_init: String, h5_groups: Vec<hdf5::Group>, h5_datasets: Vec<hdf5::Dataset>) -> Self {
        // passes in the associated groups that are children to this node
        let key = key_init;
        let mut groups = Vec::new();
        for entry in h5_groups { // populates the group within this group
            groups.push(GroupStruct::new(entry.name(), entry.groups().unwrap(), entry.datasets().unwrap()));
        }
        let mut data_set1D = Vec::new();
        let mut data_set2D = Vec::new();
        let mut data_set3D = Vec::new();
        for entry in h5_datasets {
            let shape = entry.shape();
         // determine the dimension of the dataset
            if shape.len() == 1 {
                data_set1D.push(DataSetStruct1D::new(entry))
            } else if shape.len() == 2 {
                data_set2D.push(DataSetStruct2D::new(entry))
            } else if shape.len() == 3 {
                data_set3D.push(DataSetStruct3D::new(entry))
            }
        }
        Self {key, groups, data_set1D,data_set2D,data_set3D}
    }

}

// 1D, 2D, 3D, pic, plain
// 1d array data set
pub struct DataSetStruct1D {
    id: String,
    data: ndarray::Array1<f64>,
}

impl DataSetStruct1D {
    pub fn new(data_set: hdf5::Dataset) -> Self {
        let id = data_set.name();
        let data = data_set.read_1d::<f64>().unwrap();
        Self {id , data}
    }
}
// 2d array data set
pub struct DataSetStruct2D {
    id: String,
    data: ndarray::Array2<f64>,
}
impl DataSetStruct2D {
    pub fn new(data_set: hdf5::Dataset) -> Self {
        let id = data_set.name();
        let data = data_set.read_2d::<f64>().unwrap();
        Self {id, data}
    }
}

// 3d array data set
pub struct DataSetStruct3D {
    id: String,
    data: ndarray::ArrayD<f64>,
}
impl DataSetStruct3D {
    pub fn new(data_set: hdf5::Dataset) -> Self {
        let id = data_set.name();
        let data = data_set.read_dyn::<f64>().unwrap();
        Self {id, data}
    }

}

fn read_hdf5() -> Result<()> {
    
    let mut b = Vec::new();
    b.push(vec!["nanowire_data.h5"]);
    b.push(vec!["info", "spectrometer_energy", "spectrometer_wavelength"]);
    b.push(vec!["nanowires", "fitting_errors", "geometry", "pic", "power_dependent_tcspc", "spectral_bottom", "spectral_top", "tcspc_bottom", "tcspc_top"]);
  

    let filename_init = "nanowire_data.h5";
    let file = File::open(filename_init).unwrap(); // open file for reading
    // created file object
    let node = FileNodeStruct::new(filename_init.to_string(), &file);
         
    // convert the FileNode into a JSON file using Serde
    //let j = serde_json::to_string(&mut node).unwrap();
     

    Ok(())
}

fn main() -> Result<()> {
    read_hdf5()?;
    Ok(())
}
