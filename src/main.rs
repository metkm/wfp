use windows::Win32::{
    Foundation::HANDLE,
    NetworkManagement::WindowsFilteringPlatform::{
        FWPM_FILTER_ENUM_TEMPLATE0, FWPM_FILTER0, FWPM_LAYER_ALE_AUTH_LISTEN_V4,
        FWPM_SESSION_FLAG_DYNAMIC, FWPM_SESSION0, FwpmEngineOpen0, FwpmFilterCreateEnumHandle0,
        FwpmFilterEnum0,
    },
    System::Rpc::RPC_C_AUTHN_WINNT,
};

fn find_matching_filters(engine_handle: &HANDLE) {
    let enum_template = FWPM_FILTER_ENUM_TEMPLATE0 {
        actionMask: 0xFFFFFFFF,
        layerKey: FWPM_LAYER_ALE_AUTH_LISTEN_V4,
        ..Default::default()
    };

    let mut enum_handle = HANDLE::default();

    let create_enum_result = unsafe {
        FwpmFilterCreateEnumHandle0(*engine_handle, Some(&enum_template), &mut enum_handle)
    };

    println!("{:?} created enum result", create_enum_result);

    // enumerate filters
    ///////////////////////
    ///////////////////////
    ///////////////////////
    ///////////////////////
    ///////////////////////

    let mut filters: *mut *mut FWPM_FILTER0 = std::ptr::null_mut();
    let mut returned_filter_count = 0;

    let filters_enum_result = unsafe {
        FwpmFilterEnum0(
            *engine_handle,
            enum_handle,
            20,
            &mut filters,
            // filters.as_mut_ptr() as *mut _ as *mut _,
            &mut returned_filter_count,
        )
    };

    if filters_enum_result != 0 {
        println!(
            "erorr getting filters enum result {:?}",
            filters_enum_result
        );
        return;
    }

    println!("returned filter count {:?}", returned_filter_count);

    let mut filter_objects = Vec::with_capacity(returned_filter_count as usize);

    for i in 0..returned_filter_count {
        let filter = unsafe { **filters.add(i as usize) };
        filter_objects.push(filter);
    }

    for filter in &filter_objects {
        let name = unsafe { filter.displayData.name.to_string() };
        println!("{:?} - {:?}", filter.filterKey, name);
    }

    /////////////////////
    /////////////////////
    /////////////////////
    /////////////////////
    /////////////////////
    /////////////////////
    /////////////////////
}

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

    find_matching_filters(&engine_handle);
}
