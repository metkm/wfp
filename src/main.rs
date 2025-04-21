mod events;
mod filters;

use windows::Win32::{
    Foundation::{HANDLE, SYSTEMTIME},
    NetworkManagement::WindowsFilteringPlatform::{
        FwpmEngineOpen0, FWPM_SESSION0, FWPM_SESSION_FLAG_DYNAMIC
    },
    System::{Rpc::RPC_C_AUTHN_WINNT, Time::FileTimeToSystemTime},
};

fn main() {
    let engine_session = FWPM_SESSION0 {
        flags: FWPM_SESSION_FLAG_DYNAMIC,
        ..Default::default()
    };

    let mut engine_handle = HANDLE::default();

    let engine_result = unsafe {
        FwpmEngineOpen0(
            None,
            RPC_C_AUTHN_WINNT,
            None,
            Some(&engine_session),
            &mut engine_handle,
        )
    };

    println!("{:?} engine open result", engine_result);

    // let Ok(filters) = filters::find_matching_filters(&engine_handle) else {
    //     return;
    // };

    // for filter in filters {
    //     let name = unsafe { filter.displayData.name.to_string() };
    //     println!("{:?}", name.unwrap())
    // }

    // get_recent_events(&engine_handle).unwrap();
    
    if let Ok(events) = events::get_recent_events(&engine_handle, 50) {
        for event in events {
            let mut system_time = SYSTEMTIME::default();

            let to_system_time_result = unsafe {
                FileTimeToSystemTime(&event.header.timeStamp, &mut system_time)
            };
            
            if to_system_time_result.is_err() {
                println!("error converting file time to system time");
                continue;
            }

            println!("{:?} - {:?} - {:?}", event.header.timeStamp, event.header.localPort, system_time);
        }
    }
}
