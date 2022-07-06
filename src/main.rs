#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result, Selection};
use ndarray::{arr2,s, Dimension};
use serde::{Deserialize, Serialize};
use serde_json::Result;

//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
//#[repr(C)]

//#[derive(H5Type, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FileNodeStruct {
    filename: String,
    file: hdf5::File,
    groups: Vec<GroupStruct>
}

impl FileNodeStruct {
    pub fn new(filename_init: String, file_init: hdf5::File) -> Self {
        let file = file_init;
        let filename = filename_init;
        let groups = Vec::new();



        let group_list = file.groups().unwrap();
        
        for entry in group_list {
            let temp_name = entry.name(); // string
            let temp_groups_child = entry.groups(); // Vec<h5 ... Group> 
            let temp_datasets = entry.datasets(); // Vec<h5 ... Dataset>


            groups.push(GroupStruct::new(entry.name(), entry.groups().unwrap(), entry.datasets().unwrap()))
        }
        Self {filename: filename_init, groups: groups, file: file}
    }

    pub fn populate_groups() {

    }

}


// end of file struct

pub struct GroupStruct {
    key: String,
    datasets: Vec<DataSet_struct>,
    groups: Vec<GroupStruct>
}

impl GroupStruct {
    pub fn new(key_init: String, h5_groups: Vec<hdf5::Group>, h5_datasets: Vec<hdf5::Dataset>) -> Self {
        // passes in the associated groups that are children to this node
        let key = key_init;
        let groups = Vec::new();
        for entry in h5_groups {
            groups.push(GroupStruct::new(entry.name(), entry.groups().unwrap(), entry.datasets().unwrap()));
        }
        let datasets = Vec::new();
        for entry in h5_datasets {
            datasets.push(DataSet_struct::new())       
        }

        Self {key, datasets, groups}
    }

}



//#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
// Data set struct

pub struct DataSet_struct {
    id: String,
    size: Vec<usize>,
    dim: i8,
    data_temp1: ndarray::Array1<f64>,
    data_temp2: ndarray::Array2<f64>,
    data_temp3: ndarray::ArrayD<f64>
}

enum Either<A,B> {
    Single(A),
    Double(B),
    //Dynamic(C)
}


fn pick_data(dim: usize, data_temp1: ndarray::Array1<f64>, data_temp2: ndarray::Array2<f64>) ->Either<ndarray::Array1<f64>, ndarray::Array2<f64>> {
    if dim == 1 {
        Either::Single(data_temp1);
    } else if dim == 2 {
         Either::Double(data_temp2);
    }
}

impl DataSet_struct {
    pub fn new(name: String, dataSet: hdf5::Dataset) -> Self { // implement ndarray for handling n
        // dimensional data - for now only implement 1d, 2d data
        let id = name;
        let size = dataSet.shape();
        let dim = size.len();
        // depending on data size read into
        if size.len() == 1 {
            let data_temp1 = dataSet.read_1d::<f64>().unwrap();
        } else if size.len() == 2 {
            let data_temp2 = dataSet.read_2d::<f64>().unwrap();
        } //else if size.len() == 3 {
        //    let data_temp3 = dataSet.read_dyn::<f64>().unwrap();
        //}

        Self {id, size, dim, data}
    }


    pub fn return_data(self) -> Either<ndarray::Array1<f64>, ndarray::Array2<f64> , ndarray::Array3<f64>> {
        use Either::{Single, Double};

        if self.dim == 1 {
            Either::single(self.data_temp1);
        } else if self.dim == 2 {
            Either::double(self.data_temp2);
        } else {
            Either::dynamic(self.data_temp3);
        }
    }

}

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
    let node = FileNodeStruct { filename: filename_init.to_string(), file: file};
    
    // convert the FileNode into a JSON file using Serde
    //let j = serde_json::to_string(&mut node).unwrap();
     

    Ok(())
}

fn main() -> Result<()> {
    read_hdf5()?;
    Ok(())
}
