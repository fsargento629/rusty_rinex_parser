//TODO:
// Simple rinex:
// extract each line of the header, and save that in a variable
// iterate through the rest of the file. Find each epoch and populate a vector of epoch results
// Each epoch is a hasmap witht the satellite ID and a struct with the observables



use core::panic;
use std::fs;

#[derive(Debug)]
enum SVID {
    GPS(u8),
    GAL(u8),
    GLOSNASS(u8),
    BEIDOU(u8),
    OTHERS(u8),
    
}
// struct to save the observables from a satellite
#[derive(Debug)]
struct Observables {
    t: String, // this should be a struct that contains time info (TOW, WN, etc)
    svid: SVID,
    pr:Vec<f64>,
    cp:Vec<f64>,
    doppler:Vec<f64>,
    signal_strength:Vec<f64>,   
}
impl Observables {
    // Constructor method (often named `new`)
    pub fn new(t: String, svid: SVID) -> Self {
        // You can initialize other fields as needed
        let pr = Vec::new();
        let cp = Vec::new();
        let doppler = Vec::new();
        let signal_strength = Vec::new();

        Observables {
            t,
            svid,
            pr,
            cp,
            doppler,
            signal_strength,
        }
    }
}
#[derive(Debug)]
struct RinexData {
    before_header : String,
    header : String, 
    meas : Vec<Vec<Observables>>,
}

impl RinexData {
    fn new() -> RinexData {
        RinexData { before_header:String::new(), header: String::new(), meas: Vec::new() }
        
    }
}

fn str2svid(sv_str:String) -> SVID {

    let mut sv_chars = sv_str.chars();
    match sv_chars.next().unwrap() {
        'G' => {SVID::GPS(sv_chars.as_str().parse::<u8>().unwrap())}
        'R' => {SVID::GLOSNASS(sv_chars.as_str().parse::<u8>().unwrap())}
        'E' => {SVID::GAL(sv_chars.as_str().parse::<u8>().unwrap())}
        'B' => {SVID::BEIDOU(sv_chars.as_str().parse::<u8>().unwrap())}
        _ => {panic!("Sat {} not recognized!!!",sv_str)}
    }
}

// takes a line from a rinex observation file
// converts that line to Obsersables
fn rinex_line2obsersables(rinex_line:String,t:String) -> Observables {
    println!("rinex_line2obsersables got line: {}",rinex_line);
    let mut data = rinex_line.split_whitespace();

    let svid:SVID = str2svid(data.next().unwrap().to_string());
    let mut observables = Observables::new(t,svid);

    // the next few lines should be like PR,CP,doppler,SignalStrength,...,
    // data columns must be a multiple of 4!
    if data.clone().count() %4 == 0 {
        let num_signals = data.clone().count() / 4;

        for _ in 0..num_signals {
            observables.pr.push(data.next().unwrap().parse::<f64>().unwrap());
            observables.cp.push(data.next().unwrap().parse::<f64>().unwrap());
            observables.doppler.push(data.next().unwrap().parse::<f64>().unwrap());
            observables.signal_strength.push(data.next().unwrap().parse::<f64>().unwrap());
        }
    }
    else {
        panic!("this line does not have the right number of columns!!! {}",rinex_line);
    }
    println!("End of rinex_line2observables: {:?}",observables);
    observables
    
}

fn rinex2data(file_path:String) -> Result<RinexData,String> { //-> Result<Vec<HashMap<SVID,Observables>>,String> {

    // get string with all the files' content
    let file_contents ;
    match fs::read_to_string(file_path) {
        Ok(contents) => {file_contents = contents; }
        Err(_) => {return Err("Error in reading file".to_string());}
    }

    // Find the header and save it
    // the header is anything 
    let before_after_and_header:Vec<&str> = file_contents.split("HEADER").collect();
    let mut rinex_data = RinexData::new();

    match before_after_and_header.len(){
        3 => {
            rinex_data.before_header = before_after_and_header[0].to_string();
            rinex_data.header = before_after_and_header[1].to_string();

        }
        _ => {return Err("The rinex file must have exactly two HEADER strings".to_string());}
    }
    let after_header = before_after_and_header[2];


    // after the header, iterate all epochs and get the data for each sat
    for epoch in after_header.split(">"){
        println!("--------{}",epoch);
        let mut epoch_data = epoch.lines();
        let epoch_time_str = epoch_data.next().unwrap().to_string();


        // debug:
        println!("epoch_data: {:?}",epoch_data);
        println!("epoch time str: {}---",epoch_time_str);
        let mut epoch_observables: Vec<Observables> = Vec::new();
        for rinex_line in epoch_data {
            println!("Parsing line: {}",rinex_line);
            let observables = rinex_line2obsersables(rinex_line.to_string(), epoch_time_str.clone());
            println!("Observables for this line are:{:?}",observables);
            epoch_observables.push(observables);
        }

        // let epoch_observables: Vec<Observables> = 
        //         epoch_data.map(|l|rinex_line2obsersables(l.to_string(),epoch_time_str.clone()) ).collect();
        println!("epoch_observables: {:?}",epoch_observables);
        rinex_data.meas.push(epoch_observables);
    }
    

   Ok(rinex_data)
}

fn main() {
    println!("Welcome to rusty rinex parser!");

    // define file to read and other constants as necesssary
    let file_name = String::from("example.rnx");


    let rinex_data = rinex2data(file_name).unwrap();


    println!("{:?}",rinex_data);

    // save the data to a .csv file in a pandas format
    


    println!("Thank you for using the rusty rinex parser!");

}

