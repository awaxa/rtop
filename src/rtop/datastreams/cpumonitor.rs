use std::collections::HashMap;
use sysinfo::{System, SystemExt, Processor, ProcessorExt};

use crate::rtop::datastreams::{datastream::SysDataStream, utils};


pub struct CPUMonitor {
    pub cpu_usage: f32,
    pub cpu_core_info: Vec<(String, f32)>, //Name, Usage
    pub cpu_usage_history: HashMap<String, Vec<f32>>, //Name, Usage
    pub cpu_temp: Option<f32>, 
    //pub cpu_temp_history: HashMap<String, Vec<f32>>,
    max_history_len: usize,
    interpolation_len: u16,
}

impl SysDataStream for CPUMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {        
        Self {
            cpu_usage: 0.0,
            cpu_core_info: Vec::new(),
            cpu_usage_history: HashMap::new(),
            cpu_temp: None,
            max_history_len: max_hist_len,
            interpolation_len: inter_len 
        }
    }

    fn poll(&mut self, system_info: &System) {
        let cpus = system_info.get_processors();

        self.cpu_core_info.clear();
        for cpu in &cpus[1..cpus.len()] {
            let info = CPUMonitor::parse_cpu_info(cpu);
            self.cpu_core_info.push(info);
            match self.cpu_core_info.last() {
                Some(entry) => {
                    let history = self.cpu_usage_history.entry(entry.0.clone()).or_insert(vec![0.0; self.max_history_len]);
                    while history.len() >= self.max_history_len {
                        history.remove(0);
                    }
                    let last = match history.last() {
                        Some(l) => l.clone(),
                        None => 0.0,
                    };
                    history.extend_from_slice(utils::interpolate::<f32>(last, entry.1, self.interpolation_len).as_slice());
                },
                None => {},
            };
        }
        self.cpu_usage = cpus[0].get_cpu_usage();
    }
}

impl CPUMonitor {
    fn parse_cpu_info(cpu: &Processor) -> (String, f32) {
        (String::from(cpu.get_name()), cpu.get_cpu_usage())
    }
}

